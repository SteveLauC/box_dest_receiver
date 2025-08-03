/*-------------------------------------------------------------------------
 *
 * static_inline_c_funcs.c
 *   Impl of the callable wrappers.
 *
 *-------------------------------------------------------------------------
 */


#include "postgres.h"

#include "access/tupdesc.h"
#include "executor/tuptable.h"
#include "utils/syscache.h"
#include "utils/lsyscache.h"

Datum
callable_slot_getattr(TupleTableSlot *slot, int attnum, bool *isnull) {
    return slot_getattr(slot, attnum, isnull);
}

FormData_pg_attribute *
callable_TupleDescAttr(TupleDesc tupdesc, int i)
{
    return TupleDescAttr(tupdesc, i);
}