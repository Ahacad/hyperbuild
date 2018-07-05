#ifndef _HDR_HYPERBUILD_STREAM_HELPER_ATTR
#define _HDR_HYPERBUILD_STREAM_HELPER_ATTR

#include "../../rule/char/attrname.c"
#include "../../rule/char/attrvalquote.c"
#include "./quoteattrval.c"
#include "./unquoteattrval.c"

#include "../../util/hbchar.h"
#include "../../util/pipe.c"

void hbsh_attr(hbu_pipe_t pipe) {
  while (1) {
    // Char matched by $attrname required at least once
    hbu_pipe_require_predicate(pipe, &hb_r_attrname_check, "attribute name");

    // Don't use hbu_pipe_accept_while_predicate as advanced checks might be needed
    hb_char_t n = hbu_pipe_peek(pipe);

    // TODO Check against UCASEATTR
    if (!hb_r_attrname_check(n)) {
      break;
    }
  }

  if (hbu_pipe_accept_if(pipe, '=')) {
    if (hb_r_attrvalquote_check(hbu_pipe_peek(pipe))) {
      // Quoted attribute value
      hbsh_quoteattrval(pipe);
    } else {
      // Unquoted attribute value
      hbsh_unquoteattrval(pipe);
    }
  }
}

#endif // _HDR_HYPERBUILD_STREAM_HELPER_ATTR