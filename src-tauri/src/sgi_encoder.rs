/*
https://paulbourke.net/dataformats/sgirgb/

------ SGI Header info

 Size   Type    Name       Description   
 
      2 bytes  short   MAGIC      IRIS image file magic number
                                  This should be decimal 474
      1 byte   char    STORAGE    Storage format
                                  0 for uncompressed
                                  1 for RLE compression
      1 byte   char    BPC        Number of bytes per pixel channel 
                                  Legally 1 or 2
      2 bytes  ushort  DIMENSION  Number of dimensions
                                  Legally 1, 2, or 3
                                  1 means a single row, XSIZE long
                                  2 means a single 2D image
                                  3 means multiple 2D images
      2 bytes  ushort  XSIZE      X size in pixels 
      2 bytes  ushort  YSIZE      Y size in pixels 
      2 bytes  ushort  ZSIZE      Number of channels
                                  1 indicates greyscale
                                  3 indicates RGB
                                  4 indicates RGB and Alpha
      4 bytes  long    PIXMIN     Minimum pixel value
                                  This is the lowest pixel value in the image
      4 bytes  long    PIXMAX     Maximum pixel value
                                  This is the highest pixel value in the image
      4 bytes  char    DUMMY      Ignored
                                  Normally set to 0
     80 bytes  char    IMAGENAME  Image name
                                  Must be null terminated, therefore at most 79 bytes
      4 bytes  long    COLORMAP   Colormap ID
                                  0 - normal mode
                                  1 - dithered, 3 mits for red and green, 2 for blue, obsolete
                                  2 - index colour, obsolete
                                  3 - not an image but a colourmap
    404 bytes  char    DUMMY      Ignored
                                  Should be set to 0, makes the header 512 bytes.

*/

/*
I had lots of trouble getting SGI .rgb to work.
image-extras supports SGI .sgi but does not extend ImageReader, and with image::open you cannot specify the type.

The fix was to treat the image as a file instead of an image.


ANOTHER BIG ISSUE: SGI counts from bottom left of image. we, i assume count, from topleft usually. so the resulting image was upside down.
*/

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn write_sgi(img : image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, path : &PathBuf) -> Result<(), String>{
    let rle_compression = false;
    let header=get_header(img.width() as u16, img.height() as u16, rle_compression);

    // get the file writer
    let result_file = File::create(path)
        .map_err(|e| format!("Failed to open base image: {}", e))?;
    let mut writer = BufWriter::new(result_file);

    writer.write_all(&header)
        .map_err(|e| format!("Failed to write header: {}", e))?;

    // TODO RLE compression
    // if rle_compression {
        // handle the offset table
        // then contents
    //}

    // verbatim ->

    // raw RGBA buffer (interleaved)
    let rgba_data = img.as_raw();
    let width = img.width() as usize;
    let height = img.height() as usize;

    // temp row buffer
    let mut row = vec![0u8; width];

    // z = 0..4 → R, G, B, A
    for z in 0..4 {
        for y in height..0 { // FLIP ORDER BECAUSE SGI starts counting at bottom left of image
            for x in 0..width {
                let idx = (y * width + x) * 4 + z; // * 4 because we want to skip the other colours on each pass
                row[x] = rgba_data[idx];
            }

            writer.write_all(&row)
                .map_err(|e| format!("Failed to write pixel data: {}", e))?;
        }
    }


    Ok(())
}

/*
We return
*/
fn get_header(width : u16, height: u16, rle_compression : bool) -> [u8;512]{
    let mut header = [0u8; 512];

    header[0..2].copy_from_slice(&474u16.to_be_bytes());    // Magic number (474)
    header[2] = if rle_compression { 1 } else { 0 };            // Storage (0 = raw, 1 = RLE)
    header[3] = 1;                                              // Bytes per channel (1 or 2)
    header[4..6].copy_from_slice(&3u16.to_be_bytes());     // Dimension (usually 3 for RGB images)
    header[6..8].copy_from_slice(&width.to_be_bytes());    // Width (xsize)
    header[8..10].copy_from_slice(&height.to_be_bytes());  // Height (ysize)
    header[10..12].copy_from_slice(&4u16.to_be_bytes());   // Channels (zsize = 4 for RGBA)
    header[12..16].copy_from_slice(&0u32.to_be_bytes());   // pixelmin 0
    header[16..20].copy_from_slice(&255u32.to_be_bytes()); // pixelmax 255
    // 4 bytes dummy
    let image_name = b"hello world...\0"; header[24..24+image_name.len()].copy_from_slice(image_name);// image name
    // colormap is already 0
    // dummy (ignore)

    header
}