# Printer Module And Text Renderings

In this article we will discuss how to use the [`printer`] module in other to create text renderings and output an image from them. You can apply the same fundamental concepts when rendering text onto a window using your graphics framework of choice.  
  
To begin we will first need a `SimplePixelFont` font as a `Layout`. In this case we will load the bytes of a font saved in a file and parse it to create the [`Layout`]:

```rs
let mut file = std::fs::OpenOptions::new()
    .read(true)
    .open("./sampleToyFont.spf")
    .unwrap();
let mut buffer: Vec<u8> = vec![];
file.read_to_end(&mut buffer).unwrap();
let font = spf::core::layout_from_data(buffer);
```

Now we can begin using the [`printer`] module to create text renderings. To start off lets use the [`Printer::from_font()`] method to create a new [`Printer`] struct. This struct will allow us to render texts very easily using the font of our choice:
```rs
// This method takes our Layout struct.
let printer = Printer::from_font(font);
```

Alright, now we can use the [`Printer.print()`] method to create a bitmap-like surface of any given text using the font we provided:
```rs
let surface = printer.print("some text");
```
If we then print the contents of the surface.data field we will get the following:
```sh

```
The above shows the pixmap of our text if it was rendered. In order to see the text rendering lets install another crate to create an image from the text render.

### `Surface`'s replace methods for Integration.