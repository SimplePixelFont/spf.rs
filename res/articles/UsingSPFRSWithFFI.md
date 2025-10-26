# `spf.rs` as a library
This article will discuss how to use `spf.rs` as a library in C. However, the same principles can be applied to any language that has an FFI which adhere to the platform specific C-ABI. This includes programming languages such as Python, Julia, Ruby, Java, WASM, C/C++, etc.

To begin you will need the `spf.rs` library binary which you can download from the [releases](https://github.com/SimplePixelFont/spf.rs/releases) section, or your can [compile spf.rs from source](https://docs.rs/spf/0.4.0/spf/articles/installing/index.html#compiling-spfrs-from-source) to obtain the library.

### Header Files
Additionally, to use `spf.rs` in C/C++ you will need to download the header files found in the [releases](https://github.com/SimplePixelFont/spf.rs/releases) section. You can then add the following in your C code:
```c
#include "libspf.h"
// You will also need this standard libary for loading libraries.
#include <dlfcn.h>
```

### Loading the Library
The first step is to load the `spf.rs` library, in C the `dlopen()` function from the `dlfcn` standard library is used. For windows this may differ
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
struct SPFLayout(*spf_core_layout_from_data)(char*, unsigned int);
struct SPFData(*spf_core_layout_to_data)(struct SPFLayout);

// We can assign the variables as follows
spf_core_layout_from_data = dlsym(handle, "spf_core_layout_from_data");
spf_core_layout_to_data = dlsym(handle, "spf_core_layout_to_data");
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
struct SPFLayout layout = spf_core_layout_from_data(buffer, file_size);

printf("---Header Data---\n");
printf("Format Version: %d\n", layout.version);
printf("Compact: %s\n", (bool)layout.compact ? "true" : "false");

printf("---Character Tables---\n");
for(int i = 0; i < layout.character_tables_length; i++) {
    printf("Character Table %d:\n", i);
    printf("  Use advance_x: %s\n", (bool)layout.character_tables[i].use_advance_x ? "true" : "false");
    printf("  Use pixmap_index: %s\n", (bool)layout.character_tables[i].use_pixmap_index ? "true" : "false");
    printf("  Constant Cluster Codepoints: %s(%d)\n",
        (bool)layout.character_tables[i].has_constant_cluster_codepoints ? "true" : "false",
        layout.character_tables[i].constant_cluster_codepoints
    );
    printf("  Pixmap Table Indexes: %s(", (bool)layout.character_tables[i].has_pixmap_table_indexes ? "true" : "false");
    for (int j = 0; j < layout.character_tables[i].pixmap_table_indexes_length; j++) {
        if (j == layout.character_tables[i].pixmap_table_indexes_length - 1) {
            printf("%d", layout.character_tables[i].pixmap_table_indexes[j]);
        } else {
            printf("%d, ", layout.character_tables[i].pixmap_table_indexes[j]);
        }
    }
    printf(")\n");
    printf("  Characters:\n");
    for (int j = 0; j < layout.character_tables[i].characters_length; j++) {
        printf("  - Character %d:\n", j);
        printf("    advance_x: %s(%d)\n",
            (bool)layout.character_tables[i].characters[j].has_advance_x ? "true" : "false",
            layout.character_tables[i].characters[j].advance_x
        );
        printf("    pixmap_index: %s(%d)\n",
            (bool)layout.character_tables[i].characters[j].has_pixmap_index ? "true" : "false",
            layout.character_tables[i].characters[j].pixmap_index
        );
        printf("    grapheme_cluster: '%s'\n", layout.character_tables[i].characters[j].grapheme_cluster);
    }
}

printf("---Color Tables---\n");
for(int i = 0; i < layout.color_tables_length; i++) {
    printf("Color Table %d:\n", i);
    printf("  Constant Alpha: %s(%d)\n",
        (bool)layout.color_tables[i].has_constant_alpha ? "true" : "false",
        layout.color_tables[i].constant_alpha
    );
    printf("  Colors:\n");
    for (int j = 0; j < layout.color_tables[i].colors_length; j++) {
        printf("  - Color %d:\n", j);
        printf("    custom_alpha: %s(%d)\n",
            (bool)layout.color_tables[i].colors[j].has_custom_alpha ? "true" : "false",
            layout.color_tables[i].colors[j].custom_alpha
        );
        printf("    r: %d\n", layout.color_tables[i].colors[j].r);
        printf("    g: %d\n", layout.color_tables[i].colors[j].g);
        printf("    b: %d\n", layout.color_tables[i].colors[j].b);
    }
}

printf("--- Pixmap Tables ---\n");
for(int i = 0; i < layout.pixmap_tables_length; i++) {
    printf("Pixmap Table %d:\n", i);
    printf("  Constant Width: %s(%d)\n",
        (bool)layout.pixmap_tables[i].has_constant_width ? "true" : "false",
        layout.pixmap_tables[i].constant_width
    );
    printf("  Constant Height: %s(%d)\n",
        (bool)layout.pixmap_tables[i].has_constant_height ? "true" : "false",
        layout.pixmap_tables[i].constant_height
    );
    printf("  Constant Bits Per Pixel: %s(%d)\n",
        (bool)layout.pixmap_tables[i].has_constant_bits_per_pixel ? "true" : "false",
        layout.pixmap_tables[i].constant_bits_per_pixel
    );
    printf("  Color Table Indexes: %s(", (bool)layout.pixmap_tables[i].has_color_table_indexes ? "true" : "false");
    for (int j = 0; j < layout.pixmap_tables[i].color_table_indexes_length; j++) {
        if (j == layout.pixmap_tables[i].color_table_indexes_length - 1) {
            printf("%d", layout.pixmap_tables[i].color_table_indexes[j]);
        } else {
            printf("%d, ", layout.pixmap_tables[i].color_table_indexes[j]);
        }
    }
    printf(")\n");
    printf("  Pixmaps:\n");
    for (int j = 0; j < layout.pixmap_tables[i].pixmaps_length; j++) {
        printf("  - Pixmap %d:\n", j);
        printf("    custom_width: %s(%d)\n",
            (bool)layout.pixmap_tables[i].pixmaps[j].has_custom_width ? "true" : "false",
            layout.pixmap_tables[i].pixmaps[j].custom_width
        );
        printf("    custom_height: %s(%d)\n",
            (bool)layout.pixmap_tables[i].pixmaps[j].has_custom_height ? "true" : "false",
            layout.pixmap_tables[i].pixmaps[j].custom_height
        );
        printf("    custom_bits_per_pixel: %s(%d)\n",
            (bool)layout.pixmap_tables[i].pixmaps[j].has_custom_bits_per_pixel ? "true" : "false",
            layout.pixmap_tables[i].pixmaps[j].custom_bits_per_pixel
        );
        printf("    data: ");
        for (int k = 0; k < layout.pixmap_tables[i].pixmaps[j].data_length; k++) {
            printf("%d ", layout.pixmap_tables[i].pixmaps[j].data[k]);
        }
        printf("\n");
    }
}
```
And now that we have a `crate::core::Layout` in C, or more precisely a `core::ffi::SPFLayout`, we can also convert it back into data:
```c
struct SPFData data = spf_core_layout_to_data(layout);

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

    handle = dlopen(SPF_LIBRARY, RTLD_LAZY);
    if (!handle) {
        printf("%s\n", dlerror());
        return 1;
    }

    printf("Loading libspf.so succeeded\n");

    struct SPFLayout(*spf_core_layout_from_data)(char*, unsigned int);
    struct SPFData(*spf_core_layout_to_data)(struct SPFLayout);

    // We can assign the variables as follows
    spf_core_layout_from_data = dlsym(handle, "spf_core_layout_from_data");
    spf_core_layout_to_data = dlsym(handle, "spf_core_layout_to_data");

    /* We can use spf.rs functions now that we have loaded and assigned them to variables */

    struct SPFLayout layout = spf_core_layout_from_data(buffer, file_size);

    printf("---Header Data---\n");
    printf("Format Version: %d\n", layout.version);
    printf("Compact: %s\n", (bool)layout.compact ? "true" : "false");

    printf("---Character Tables---\n");
    for(int i = 0; i < layout.character_tables_length; i++) {
        printf("Character Table %d:\n", i);
        printf("  Use advance_x: %s\n", (bool)layout.character_tables[i].use_advance_x ? "true" : "false");
        printf("  Use pixmap_index: %s\n", (bool)layout.character_tables[i].use_pixmap_index ? "true" : "false");
        printf("  Constant Cluster Codepoints: %s(%d)\n",
            (bool)layout.character_tables[i].has_constant_cluster_codepoints ? "true" : "false",
            layout.character_tables[i].constant_cluster_codepoints
        );
        printf("  Pixmap Table Indexes: %s(", (bool)layout.character_tables[i].has_pixmap_table_indexes ? "true" : "false");
        for (int j = 0; j < layout.character_tables[i].pixmap_table_indexes_length; j++) {
            if (j == layout.character_tables[i].pixmap_table_indexes_length - 1) {
                printf("%d", layout.character_tables[i].pixmap_table_indexes[j]);
            } else {
                printf("%d, ", layout.character_tables[i].pixmap_table_indexes[j]);
            }
        }
        printf(")\n");
        printf("  Characters:\n");
        for (int j = 0; j < layout.character_tables[i].characters_length; j++) {
            printf("  - Character %d:\n", j);
            printf("    advance_x: %s(%d)\n",
                (bool)layout.character_tables[i].characters[j].has_advance_x ? "true" : "false",
                layout.character_tables[i].characters[j].advance_x
            );
            printf("    pixmap_index: %s(%d)\n",
                (bool)layout.character_tables[i].characters[j].has_pixmap_index ? "true" : "false",
                layout.character_tables[i].characters[j].pixmap_index
            );
            printf("    grapheme_cluster: '%s'\n", layout.character_tables[i].characters[j].grapheme_cluster);
        }
    }

    printf("---Color Tables---\n");
    for(int i = 0; i < layout.color_tables_length; i++) {
        printf("Color Table %d:\n", i);
        printf("  Constant Alpha: %s(%d)\n",
            (bool)layout.color_tables[i].has_constant_alpha ? "true" : "false",
            layout.color_tables[i].constant_alpha
        );
        printf("  Colors:\n");
        for (int j = 0; j < layout.color_tables[i].colors_length; j++) {
            printf("  - Color %d:\n", j);
            printf("    custom_alpha: %s(%d)\n",
                (bool)layout.color_tables[i].colors[j].has_custom_alpha ? "true" : "false",
                layout.color_tables[i].colors[j].custom_alpha
            );
            printf("    r: %d\n", layout.color_tables[i].colors[j].r);
            printf("    g: %d\n", layout.color_tables[i].colors[j].g);
            printf("    b: %d\n", layout.color_tables[i].colors[j].b);
        }
    }

    printf("--- Pixmap Tables ---\n");
    for(int i = 0; i < layout.pixmap_tables_length; i++) {
        printf("Pixmap Table %d:\n", i);
        printf("  Constant Width: %s(%d)\n",
            (bool)layout.pixmap_tables[i].has_constant_width ? "true" : "false",
            layout.pixmap_tables[i].constant_width
        );
        printf("  Constant Height: %s(%d)\n",
            (bool)layout.pixmap_tables[i].has_constant_height ? "true" : "false",
            layout.pixmap_tables[i].constant_height
        );
        printf("  Constant Bits Per Pixel: %s(%d)\n",
            (bool)layout.pixmap_tables[i].has_constant_bits_per_pixel ? "true" : "false",
            layout.pixmap_tables[i].constant_bits_per_pixel
        );
        printf("  Color Table Indexes: %s(", (bool)layout.pixmap_tables[i].has_color_table_indexes ? "true" : "false");
        for (int j = 0; j < layout.pixmap_tables[i].color_table_indexes_length; j++) {
            if (j == layout.pixmap_tables[i].color_table_indexes_length - 1) {
                printf("%d", layout.pixmap_tables[i].color_table_indexes[j]);
            } else {
                printf("%d, ", layout.pixmap_tables[i].color_table_indexes[j]);
            }
        }
        printf(")\n");
        printf("  Pixmaps:\n");
        for (int j = 0; j < layout.pixmap_tables[i].pixmaps_length; j++) {
            printf("  - Pixmap %d:\n", j);
            printf("    custom_width: %s(%d)\n",
                (bool)layout.pixmap_tables[i].pixmaps[j].has_custom_width ? "true" : "false",
                layout.pixmap_tables[i].pixmaps[j].custom_width
            );
            printf("    custom_height: %s(%d)\n",
                (bool)layout.pixmap_tables[i].pixmaps[j].has_custom_height ? "true" : "false",
                layout.pixmap_tables[i].pixmaps[j].custom_height
            );
            printf("    custom_bits_per_pixel: %s(%d)\n",
                (bool)layout.pixmap_tables[i].pixmaps[j].has_custom_bits_per_pixel ? "true" : "false",
                layout.pixmap_tables[i].pixmaps[j].custom_bits_per_pixel
            );
            printf("    data: ");
            for (int k = 0; k < layout.pixmap_tables[i].pixmaps[j].data_length; k++) {
                printf("%d ", layout.pixmap_tables[i].pixmaps[j].data[k]);
            }
            printf("\n");
        }
    }

    struct SPFData data = spf_core_layout_to_data(layout);

    printf("Data: ");
    for (int i = 0; i < data.data_length; i++) {
        printf("%d ", data.data[i]);
    }
    printf("\n");

    return 0;
}
```
