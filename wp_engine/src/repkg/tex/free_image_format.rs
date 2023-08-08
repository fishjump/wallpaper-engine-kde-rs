use std::u8;

use anyhow::Result;
use num_enum::TryFromPrimitive;

use crate::wp_result;

#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum FreeImageFormat {
    /// <summary>
    /// Unknown format (returned value only, never use it as input value)
    /// </summary>
    FifUnknown = u8::MAX,

    /// <summary>
    /// Windows or OS/2 Bitmap File (*.BMP)
    /// </summary>
    FifBmp = 0,

    /// <summary>
    /// Windows Icon (*.ICO)
    /// </summary>
    FifIco = 1,

    /// <summary>
    /// Independent JPEG Group (*.JPG, *.JIF, *.JPEG, *.JPE)
    /// </summary>
    FifJpeg = 2,

    /// <summary>
    /// JPEG Network Graphics (*.JNG)
    /// </summary>
    FifJng = 3,

    /// <summary>
    /// Commodore 64 Koala format (*.KOA)
    /// </summary>
    FifKoala = 4,

    /// <summary>
    /// Amiga IFF (*.IFF, *.LBM)
    /// </summary>
    FifLbmIff = 5,

    /// <summary>
    /// Multiple Network Graphics (*.MNG)
    /// </summary>
    FifMng = 6,

    /// <summary>
    /// Portable Bitmap (ASCII) (*.PBM)
    /// </summary>
    FifPbm = 7,

    /// <summary>
    /// Portable Bitmap (BINARY) (*.PBM)
    /// </summary>
    FifPbmraw = 8,

    /// <summary>
    /// Kodak PhotoCD (*.PCD)
    /// </summary>
    FifPcd = 9,

    /// <summary>
    /// Zsoft Paintbrush PCX bitmap format (*.PCX)
    /// </summary>
    FifPcx = 10,

    /// <summary>
    /// Portable Graymap (ASCII) (*.PGM)
    /// </summary>
    FifPgm = 11,

    /// <summary>
    /// Portable Graymap (BINARY) (*.PGM)
    /// </summary>
    FifPgmraw = 12,

    /// <summary>
    /// Portable Network Graphics (*.PNG)
    /// </summary>
    FifPng = 13,

    /// <summary>
    /// Portable Pixelmap (ASCII) (*.PPM)
    /// </summary>
    FifPpm = 14,

    /// <summary>
    /// Portable Pixelmap (BINARY) (*.PPM)
    /// </summary>
    FifPpmraw = 15,

    /// <summary>
    /// Sun Rasterfile (*.RAS)
    /// </summary>
    FifRas = 16,

    /// <summary>
    /// truevision Targa files (*.TGA, *.TARGA)
    /// </summary>
    FifTarga = 17,

    /// <summary>
    /// Tagged Image File Format (*.TIF, *.TIFF)
    /// </summary>
    FifTiff = 18,

    /// <summary>
    /// Wireless Bitmap (*.WBMP)
    /// </summary>
    FifWbmp = 19,

    /// <summary>
    /// Adobe Photoshop (*.PSD)
    /// </summary>
    FifPsd = 20,

    /// <summary>
    /// Dr. Halo (*.CUT)
    /// </summary>
    FifCut = 21,

    /// <summary>
    /// X11 Bitmap Format (*.XBM)
    /// </summary>
    FifXbm = 22,

    /// <summary>
    /// X11 Pixmap Format (*.XPM)
    /// </summary>
    FifXpm = 23,

    /// <summary>
    /// DirectDraw Surface (*.DDS)
    /// </summary>
    FifDds = 24,

    /// <summary>
    /// Graphics Interchange Format (*.GIF)
    /// </summary>
    FifGif = 25,

    /// <summary>
    /// High Dynamic Range (*.HDR)
    /// </summary>
    FifHdr = 26,

    /// <summary>
    /// Raw Fax format CCITT G3 (*.G3)
    /// </summary>
    FifFaxg3 = 27,

    /// <summary>
    /// Silicon Graphics SGI image format (*.SGI)
    /// </summary>
    FifSgi = 28,

    /// <summary>
    /// OpenEXR format (*.EXR)
    /// </summary>
    FifExr = 29,

    /// <summary>
    /// JPEG-2000 format (*.J2K, *.J2C)
    /// </summary>
    FifJ2k = 30,

    /// <summary>
    /// JPEG-2000 format (*.JP2)
    /// </summary>
    FifJp2 = 31,

    /// <summary>
    /// Portable FloatMap (*.PFM)
    /// </summary>
    FifPfm = 32,

    /// <summary>
    /// Macintosh PICT (*.PICT)
    /// </summary>
    FifPict = 33,

    /// <summary>
    /// RAW camera image (*.*)
    /// </summary>
    FifRaw = 34,
}

impl FreeImageFormat {
    pub fn wp_try_from(value: u32) -> Result<Self> {
        let value = value as u8;
        let format = Self::try_from(value);

        match format {
            Ok(x) => Ok(x),
            Err(_) => wp_result!(
                RepkgInvalidFreeImageFormat,
                "a value between [0, 34]",
                value
            ),
        }
    }
}
