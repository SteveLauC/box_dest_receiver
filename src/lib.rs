mod pg_sys;
mod styled_table;

use pg_sys::callable_TupleDescAttr;
use pg_sys::callable_slot_getattr;
use pg_sys::CommandDest;
use pg_sys::DestReceiver;
use pg_sys::TupleDesc;
use pg_sys::TupleTableSlot;

use comfy_table::Table;
use libc::c_int;
use std::ffi::CStr;

/// A DestReceiver implementation inspired by DuckDB's Box Renderer.
//
// Rust compiler could rearrange fields order, use C repr to prevent that.
#[repr(C)]
struct BoxDestReceiver {
    /// This should be the first field so that we can do pointer casting.
    ///
    /// * mut BoxDestReceiver -> *mut DestReceiver
    dr: DestReceiver,
    table: Table,
}

#[no_mangle]
pub extern "C" fn box_dr_create(dest: CommandDest) -> *mut DestReceiver {
    let dr = DestReceiver {
        receiveSlot: Some(box_dr_receive_slot),
        rStartup: Some(box_dr_startup),
        rShutdown: Some(box_dr_shutdown),
        rDestroy: Some(box_dr_destroy),
        mydest: dest,
    };
    let table = styled_table::styled_table();

    let box_dr = BoxDestReceiver { dr, table };
    let box_ptr = Box::new(box_dr);

    let raw_ptr = Box::into_raw(box_ptr);

    raw_ptr.cast()
}

extern "C" fn box_dr_startup(self_: *mut DestReceiver, _operation: c_int, typeinfo: TupleDesc) {
    let self_ = self_.cast::<BoxDestReceiver>();
    let n_columns = unsafe { (*typeinfo).natts };
    let mut columns = Vec::with_capacity(n_columns as usize);

    for idx in 0..n_columns {
        let form_data_pg_attribute = unsafe { callable_TupleDescAttr(typeinfo, idx) };
        let attname = unsafe { *form_data_pg_attribute }.attname;
        let cstr_ptr = attname.data.as_ptr();
        let cstr = unsafe { CStr::from_ptr(cstr_ptr) };

        // self_.table will outlive this function and typeinfo, we need to clone the
        // returned string.
        let str = cstr.to_string_lossy().into_owned();

        columns.push(str);
    }

    unsafe { (*self_).table.set_header(columns) };
}

extern "C" fn box_dr_receive_slot(slot: *mut TupleTableSlot, self_: *mut DestReceiver) -> bool {
    let self_ = self_.cast::<BoxDestReceiver>();
    let table = unsafe { &mut (*self_).table };

    let typeinfo = unsafe { (*slot).tts_tupleDescriptor };
    let natts = unsafe { (*typeinfo).natts };
    let mut row = Vec::new();

    for i in 0..natts {
        let mut is_null = false;
        let attr = unsafe { callable_slot_getattr(slot, i + 1, &mut is_null) };

        if is_null {
            row.push("NULL".to_string());
        } else {
            let mut typoutput = 0;
            let mut typisvarlena = false;
            unsafe {
                pg_sys::getTypeOutputInfo(
                    callable_TupleDescAttr(typeinfo, i)
                        .as_ref()
                        .unwrap()
                        .atttypid,
                    &mut typoutput,
                    &mut typisvarlena,
                );
            }

            let value = unsafe { CStr::from_ptr(pg_sys::OidOutputFunctionCall(typoutput, attr)) };
            row.push(value.to_string_lossy().into_owned());
        }
    }
    table.add_row(row);
    true
}

extern "C" fn box_dr_shutdown(self_: *mut DestReceiver) {
    let self_ = self_.cast::<BoxDestReceiver>();

    unsafe {
        println!("{}", (*self_).table);
    }
}

extern "C" fn box_dr_destroy(self_: *mut DestReceiver) {
    let raw_ptr = self_.cast::<BoxDestReceiver>();
    let box_ptr = unsafe { Box::from_raw(raw_ptr) };

    // Free the memory
    drop(box_ptr)
}
