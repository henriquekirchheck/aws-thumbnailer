use hypertext::prelude::*;

#[component]
pub fn index() -> impl Renderable {
    maud! {
      article {
        header {
          h1 { "Upload your image!" }
        }
        form {
          label for="file" { "Choose Image to Upload" }
          input #file name="file" type="file" accept="image/avif,image/bmp,image/dds,image/gif,image/hdr,image/ico,image/jpeg,image/jpg,image/exr,image/png,image/pnm,image/qoi,image/tga,image/tiff,image/webp";
          button type="submit" { "Submit" }
        }
      }
    }
}
