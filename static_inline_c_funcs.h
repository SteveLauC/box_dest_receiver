/*-------------------------------------------------------------------------
 *
 * static_inline_c_funcs.h
 *    We cannot access Postgres's static functions as their symbols are not
 *    exported.  So we wrap them and export these wrappers here.
 *
 *-------------------------------------------------------------------------
 */

#include "postgres.h"

#include "access/tupdesc.h"

Datum
callable_slot_getattr(TupleTableSlot *slot, int attnum, bool *isnull);


FormData_pg_attribute *
callable_TupleDescAttr(TupleDesc tupdesc, int i);