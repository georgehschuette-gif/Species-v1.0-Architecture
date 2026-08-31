#include "graceful_failure.h"
#include <cstdio>

namespace omega {

void GracefulFailure::respond(char* out, int n, const char* suggestion) const {
  std::snprintf(out, n, "I don't know. Suggestion: %s", suggestion ? suggestion : "");
}

}  // namespace omega
