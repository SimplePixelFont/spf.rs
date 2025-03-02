# `spf.rs` as a library
In this article we will discuss how to use `spf.rs` as a library in C, however the same principles can be applied to any language that has a FFI which adhere to the platform specific C-ABI. This includes programming languages such as Python, Julia, Ruby, Java, WASM, C/C++, etc.

To being you will need the binary version of the `spf.rs` library which you can download from the [releases](https://github.com/SimplePixelFont/spf.rs/releases) section or your can [compile spf.rs from source](https://docs.rs/spf/0.4.0/spf/articles/installing/index.html#compiling-spfrs-from-source) to obtain the library.

### Header Files
If you plan to use `spf.rs` in C/C++ you will also need to download the header file which can be found in the [releases](https://github.com/SimplePixelFont/spf.rs/releases) section. Note that the structs in the header file use an `SPF` prefix instead of `FFI`. You can then add the following in your C code:
```c
#include "libspf.h"
// You will also need this standard libary for loading libraries.
#include <dlfcn.h>
```

### Loading the Library
The first step is to load the `spf.rs` library, in C we use the `dlopen()` function from the `dlfcn` standard library.
```c
printf("Loading libspf.so\n");

void* handle;
handle = dlopen("/path/to/libspf.so", RTLD_LAZY);
if (!handle) {
    printf("%s\n", dlerror());
    return 1;
}

printf("Loading libspf.so succeeded\n");
```

### Define symbols
Next we need to store the function symbols from the library into variables so we can use them in our program:
```c
struct SPFLayout(*c_core_layout_from_data)(char*, unsigned int);
struct SPFData(*c_core_layout_to_data)(struct SPFLayout);

// We can assign the variables as follows
c_core_layout_from_data = dlsym(handle, "c_core_layout_from_data");
c_core_layout_to_data = dlsym(handle, "c_core_layout_to_data");
```
### Extra
We can now use the symbols we defined and begin calling `spf.rs` functions. However for our example, here is also a function that loads a file into a buffer in C. We will use this in the next step:
```c
int read_file_to_buffer(char **buffer, unsigned int *file_size) {
    FILE *file;

    file = fopen("path/to/font.spf", "rb");
    if (file == NULL) {
        printf("Error opening file");
        return 1;
    }

    fseek(file, 0, SEEK_END);
    *file_size = ftell(file);
    rewind(file);

    *buffer = (char*)malloc(*file_size + 1);
    if (*buffer == NULL) {
        printf("Memory allocation failed");
        fclose(file);
        return 1;
    }

    size_t bytes_read = fread(*buffer, 1, *file_size, file);

    if (bytes_read != (size_t)*file_size) {
        printf("Error reading file");
        free(*buffer);
        fclose(file);
        return 1;
    }

    (*buffer)[*file_size] = '\0';

    fclose(file);

    return 0;
}
```
### Calling `spf.rs` functions
Now that we have our symbols defined, here is a simple script that uses the above function to load a `spf.rs` file and extract all the fields / characters:
```c
struct SPFLayout layout = (*c_core_layout_from_data)(buffer, file_size);

printf("Alignment: %d\n", layout.header.configuration_flags.alignment);
printf("Compact: %d\n", layout.header.modifier_flags.compact);
printf("Constant size: %d\n", layout.header.required_values.constant_size);

for (int i = 0; i < layout.body.characters_length; i++) {
    printf("Loaded character with index %d:\n", i);
    printf("    custom_size: %d - character: '%s' - byte_map: ",
            layout.body.characters[i].custom_size, layout.body.characters[i].utf8);
    for (int j = 0; j < layout.body.characters[i].byte_map_length; j++) {
        printf("%d ", layout.body.characters[i].byte_map[j]);
    }
    printf("\n");
}
```
And now that we have a `crate::core::Layout` in C, or more precisely a `core::c::CLayout`, we can also convert it back into data:
```c
struct SPFData data = (*c_core_layout_to_data)(layout);

printf("Data: ");
for (int i = 0; i < data.data_length; i++) {
    printf("%d ", data.data[i]);
}
printf("\n");
```
### Full code for Copy & Paste
Minimal Changes: Filepaths are now defined as constants for even more Copy & Paste friendliness.
```c
#include <stdio.h>
#include <dlfcn.h>
#include <stdlib.h>
#include "libspf.h"

#define FILENAME "./FGMiniscript (Compact).spf"
#define SPF_LIBRARY "./libspf.so"

int read_file_to_buffer(char **buffer, unsigned int *file_size) {
    FILE *file;

    file = fopen(FILENAME, "rb");
    if (file == NULL) {
        printf("Error opening file");
        return 1;
    }

    fseek(file, 0, SEEK_END);
    *file_size = ftell(file);
    rewind(file);

    *buffer = (char*)malloc(*file_size + 1);
    if (*buffer == NULL) {
        printf("Memory allocation failed");
        fclose(file);
        return 1;
    }

    size_t bytes_read = fread(*buffer, 1, *file_size, file);

    if (bytes_read != (size_t)*file_size) {
        printf("Error reading file");
        free(*buffer);
        fclose(file);
        return 1;
    }

    (*buffer)[*file_size] = '\0';

    fclose(file);

    return 0;
}

int main() {
    char *buffer;
    unsigned int file_size;

    int result = read_file_to_buffer(&buffer, &file_size);
    if (result != 0) {
        return result;
    }

    printf("Loading libspf.so\n");

    void* handle;
    struct SPFLayout(*c_core_layout_from_data)(char*, unsigned int);
    struct SPFData(*c_core_layout_to_data)(struct SPFLayout);

    handle = dlopen(SPF_LIBRARY, RTLD_LAZY);
    if (!handle) {
        printf("%s\n", dlerror());
        return 1;
    }

    printf("Loading libspf.so succeeded\n");

    c_core_layout_from_data = dlsym(handle, "c_core_layout_from_data");
    c_core_layout_to_data = dlsym(handle, "c_core_layout_to_data");

    /* We can use spf.rs functions now that we have loaded and assigned them to variables */

    struct SPFLayout layout = (*c_core_layout_from_data)(buffer, file_size);

    printf("Alignment: %d\n", layout.header.configuration_flags.alignment);
    printf("Compact: %d\n", layout.header.modifier_flags.compact);
    printf("Constant size: %d\n", layout.header.required_values.constant_size);

    for (int i = 0; i < layout.body.characters_length; i++) {
        printf("Loaded character with index %d:\n", i);
        printf("    custom_size: %d - character: '%s' - byte_map: ",
            layout.body.characters[i].custom_size, layout.body.characters[i].utf8);
        for (int j = 0; j < layout.body.characters[i].byte_map_length; j++) {
            printf("%d ", layout.body.characters[i].byte_map[j]);
        }
        printf("\n");
    }

    struct SPFData data = (*c_core_layout_to_data)(layout);

    printf("Data: ");
    for (int i = 0; i < data.data_length; i++) {
        printf("%d ", data.data[i]);
    }
    printf("\n");

    return 0;
}
```
