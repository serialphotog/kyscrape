use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::landform;

// GPX parts
const XML_HEAD: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n";
const GPX_OPEN: &str = "<gpx xmlns=\"http://www.topografix.com/GPX/1/1\" creator=\"KentuckyLandformDatabase\" version=\"1.1\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd\">\n";
const GPX_CLOSE: &str = "</gpx>\n";

/// Writes a vector of landforms out to a GPX file on disk
///
/// This is a terrible GPX writer implementation that nobody should every use :D
pub fn write_gpx(path: &Path, landforms: &Vec<landform::Landform>) -> Result<(), std::io::Error>
{
    let mut file = File::create(path).expect("Failed to create GPX file!");

    // Write the GPX preamble   
    file.write_all(XML_HEAD.as_bytes())?;
    file.write_all(GPX_OPEN.as_bytes())?;

    // Write the waypoints
    for waypoint in landforms {
       let mut wpt_tag = "\t<wpt lat=\"".to_owned();
       wpt_tag = wpt_tag + &waypoint.latitude[..] + "\" lon=\"";
       wpt_tag = wpt_tag + &waypoint.longitude[..] + "\">\n";
       file.write_all(wpt_tag.as_bytes())?;
       file.write_all(b"\t\t<name>")?;
       file.write_all(waypoint.name.as_bytes())?;
       file.write_all(b"</name>\n")?;
       file.write_all(b"\t</wpt>\n")?;
    }
    
    // Close this thing down!
    file.write_all(GPX_CLOSE.as_bytes())?;
    Ok(())
}