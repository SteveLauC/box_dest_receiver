/*-------------------------------------------------------------------------
 *
 * box_dest_receiver.h
 *    A DestReceiver implementation that renders query results in a box 
 *    format, inspired by DuckDB's box renderer. 
 *    It is meant to be used under the single-user mode so that it writes 
 *    to stdout. 
 *-------------------------------------------------------------------------
 */


#include "postgres.h"

#include "tcop/dest.h"


DestReceiver * box_dr_create(CommandDest dest); 