// Include the standard definitions header from the standard library, so that we
// have access to 'NULL'. This can be removed if your changes remove the need
// for 'NULL'.
#include <stddef.h>

#include "hello_world.h"

// Define the function itself.
const char *hello(void)
{
   char *answer = "Hello, World!";
   return answer;
}
