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
