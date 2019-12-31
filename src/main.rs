extern crate clap;
extern crate serde;

use clap::App;
use csv::StringRecord;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// Author: Frank A. Stevenson
// Copyright 2019
// See attached MIT & APACHE licence files for terms of use

// Struct that allows "Contact Manager" contacts CVS files to be deserialised with serde
#[derive(Debug, Deserialize)]
struct CmContact {
    dmr_id: u32,       // "2429135"
    call_name: String, // "LA5AUA Stefan "
    call_type: String, // "Private Call"
    alert: String,     // "No"
    _ignore1: String,  // ""
    _ignore2: String,  // ""
    _ignore3: String,  // ""
    _ignore4: String,  // ""
    _ignore5: String,  // ""
}

// Struct that allows "Contact Manager" channel CVS files to be deserialised with serde 
// Most fields are not named / identified yet, only the most important ones have been.
#[derive(Debug, Deserialize)]
struct CmChannel {
    name: String,      // "433.400 FMN"
    mode: String,      // "FM"
    bwidth: f64,       // "12.5"
    tx_freq: f64,      // "433.400000"
    rx_freq: f64,      // "433.400000"
    _ignore1: String,  // "-NULL-"
    _ignore2: String,  // "NORMAL"
    _ignore3: String,  // "Channel Free"
    _ignore4: String,  // "Medium"
    _ignore5: String,  // "Medium"
    _ignore6: String,  // "90"
    _ignore7: String,  // "0"
    power: String,     // "HIGH"
    _ignore8: String,  // "No"
    _ignore9: String,  // "No"
    _ignore10: String, // "No"
    _ignore11: String, // "No"
    _ignore12: String, // "Yes"
    ctcss_r: String,   // "NONE"
    ctcss_t: String,   // "NONE"
    _ignore15: String, // "180"
    _ignore16: String, // "Off
    _ignore17: String, // "Off"
    _ignore18: String, // "YES"
    _ignore19: String, // "NO"
    _ignore20: String, // "NO"
    _ignore21: String, // "NO"
    _ignore22: String, // "NO"
    _ignore23: String, // "NO"
    _ignore24: String, // "NO"
    _ignore25: String, // "NO"
    _ignore26: String, // "NO"
    _ignore27: String, // "YES"
    _ignore28: String, // "NO"
    _ignore29: String, // "YES"
    _ignore30: String, // "NONE"
    group_id: String,  // "NONE"
    _ignore32: String, // "NONE"
    colour: i32,       // "1"
    _ignore34: String, // "NONE"
    _ignore35: String, // "16"
    slot: i32,         // "2"
}

// Read a "Contact Manager" contacts file
fn read_contacts(filename: &str) -> Result<Vec<CmContact>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    rdr.set_headers(StringRecord::from(vec![
        "dmr_id",
        "call_name",
        "call_type",
        "alert",
        "_ignore1",
        "_ignore2",
        "_ignore3",
        "_ignore4",
        "_ignore5",
    ]));
    let mut contacts = Vec::new();
    for result in rdr.deserialize() {
        let record: CmContact = result?;
        contacts.push(record)
    }
    Ok(contacts)
}

// Double quote a vector if str
fn quote(ins: &[&str]) -> Vec<String> {
    ins.iter().map(|s| format!("\"{}\"", s)).collect()
}

// Write contacts in a format that is readable by DJ-MD% CSP utility
fn write_contacts(filename: &str, contacts: &[CmContact]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    let fields = vec![
        "No.",
        "Radio ID",
        "Callsign",
        "Name",
        "City",
        "State",
        "Country",
        "Remarks",
        "Call Type",
        "Call Alert",
    ];
    let f2 = quote(&fields).join(",");
    file.write_all(f2.as_bytes())?;
    file.write_all(b"\r\n")?;
    for (i, cnt) in contacts.iter().enumerate() {
        let call_name = cnt.call_name.trim();
        let call_name = match call_name.find(' ') {
            None => (call_name, call_name),
            Some(p) => (&call_name[..p], &call_name[(p + 1)..]),
        };
        let ent = format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\r\n",
            i + 1,
            cnt.dmr_id,
            call_name.0,
            call_name.1,
            "",
            "",
            "",
            "",
            cnt.call_type,
            "None"
        );
        file.write_all(ent.as_bytes())?;
    }
    Ok(())
}

// Read a "Contact Manager" channel file
fn read_channels(filename: &str) -> Result<Vec<CmChannel>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    let fields = vec![
        "name",
        "mode",
        "bwidth",
        "tx_freq",
        "rx_freq",
        "_ignore1",
        "_ignore2",
        "_ignore3",
        "_ignore4",
        "_ignore5",
        "_ignore6",
        "_ignore7",
        "power",
        "_ignore8",
        "_ignore9",
        "_ignore10",
        "_ignore11",
        "_ignore12",
        "ctcss_r",
        "ctcss_t",
        "_ignore15",
        "_ignore16",
        "_ignore17",
        "_ignore18",
        "_ignore19",
        "_ignore20",
        "_ignore21",
        "_ignore22",
        "_ignore23",
        "_ignore24",
        "_ignore25",
        "_ignore26",
        "_ignore27",
        "_ignore28",
        "_ignore29",
        "_ignore30",
        "group_id",
        "_ignore32",
        "colour",
        "_ignore34",
        "_ignore35",
        "slot",
    ];
    rdr.set_headers(StringRecord::from(fields));
    let mut channels = Vec::new();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        // let record: Option<CmContact> = match result {
        //    Ok(r) => Some(r),
        //    Err(_e) => None
        //};
        let chan: CmChannel = result?;
        channels.push(chan)
        //println!("{:?}", record);
    }
    Ok(channels)
}

// Write channels in a format that is readable by DJ-MD% CSP utility
// Generate a group file if requestied, with 1 group pr DMR_ID
// A sample zone file is created by joining roughly equally sized groups of DMR_IDS
fn write_channels(
    filename: &str,
    channels: &[CmChannel],
    groupname: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    let fields = vec![
        "No.",
        "Channel Name",
        "Receive Frequency",
        "Transmit Frequency",
        "Channel Type",
        "Transmit Power",
        "Band Width",
        "CTCSS/DCS Decode",
        "CTCSS/DCS Encode",
        "Contact",
        "Contact Call Type",
        "Contact TG/DMR ID",
        "Radio ID",
        "Busy Lock/TX Permit",
        "Squelch Mode",
        "Optional Signal",
        "DTMF ID",
        "2Tone ID",
        "5Tone ID",
        "PTT ID",
        "Color Code",
        "Slot",
        "Scan List",
        "Receive Group List",
        "TX Prohibit",
        "Reverse",
        "Simplex TDMA",
        "TDMA Adaptive",
        "Encryption Type",
        "Digital Encryption",
        "Call Confirmation",
        "Talk Around",
        "Work Alone",
        "Custom CTCSS",
        "2TONE Decode",
        "Ranging",
        "Through Mode",
    ];
    let f2 = quote(&fields).join(",");
    file.write_all(f2.as_bytes())?;
    file.write_all(b"\r\n")?;
    let mut groups: HashMap<u32, String> = HashMap::new();
    let mut zones: HashMap<String, Vec<String>> = HashMap::new();
    for (i, ch) in channels.iter().enumerate() {
        let modulation = match ch.mode.as_str() {
            "DMR" => &"D-Digital",
            _ => &"A-Analog",
        };
        let power = match ch.power.as_str() {
            "LOW" => "Low",
            "MEDIUM" => "Mid",
            _ => "Turbo",
        };
        let ctcss_r = match ch.ctcss_r.as_str() {
            "None" => String::from("Off"),
            "000.0" => String::from("Off"),
            s => String::from(s),
        };
        let ctcss_t = match ch.ctcss_t.as_str() {
            "None" => String::from("Off"),
            "000.0" => String::from("Off"),
            s => String::from(s),
        };
        // Ensure unique mapping contact_dmr_id -> contact_name
        let num: Vec<&str> = ch.group_id.split(' ').collect();
        let contact: String = if !ch.group_id.is_empty() {
            match num[0].parse::<u32>() {
                Ok(n) => {
                    if groups.contains_key(&n) {
                        groups.get(&n).unwrap().clone() // Copy existing
                    } else {
                        let _ = groups.insert(n, ch.group_id.to_owned());
                        ch.group_id.to_owned() // Return existing
                    }
                }
                Err(_e) => String::from(""),
            }
        } else {
            String::from("")
        };
        // add to zone
        let zone = if !num[0].is_empty() {
            num[0].to_owned()
        } else {
            String::from("Default")
        }; // format!("{:.3}", ch.rx_freq );
        match zones.get_mut(&zone) {
            Some(z) => z.push(ch.name.to_owned()),
            None => {
                let v = vec![ch.name.to_owned()];
                let _ = zones.insert(zone, v);
            }
        }
        let ent = format!("\"{}\",\"{}\",\"{:.5}\",\"{:.5}\",\"{}\",\"{}\",\"{}K\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\r\n",
            i+1, ch.name, ch.rx_freq, ch.tx_freq, modulation,
            power, ch.bwidth, ctcss_r, ctcss_t, contact, "Group Call",
            ch.group_id, "","Always","Carrier","Off","1",
            "1","1","Off",ch.colour, ch.slot,"None","None","Off",
            "Off","Off","Off","Normal Encryption","Off","Off",
            "Off","Off","251.1","0","Off","Off");
        file.write_all(ent.as_bytes())?;
    }
    println!("Saved {} channels", channels.len());
    let mut i = 1;
    if let Some(gname) = groupname {
        let mut file = File::create(gname)?;
        let fields = vec!["No.", "Radio ID", "Name", "Call Type", "Call Alert"];
        let f2 = quote(&fields).join(",");
        file.write_all(f2.as_bytes())?;
        file.write_all(b"\r\n")?;
        for (k, v) in groups.iter() {
            let ent = format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\r\n",
                i, k, v, "Group Call", "None"
            );
            file.write_all(ent.as_bytes())?;
            i += 1;
        }
        println!("Saved {} groups", groups.len());
    }
    // Create Zones
    {
        let mut file = File::create("zone.csv")?;
        let fields = vec![
            "No.",
            "Zone Name",
            "Zone Channel Member",
            "A Channel",
            "B Channel",
        ];
        let f2 = quote(&fields).join(",");
        file.write_all(f2.as_bytes())?;
        file.write_all(b"\r\n")?;
        // Rebalance the zones
        loop {
            let mut clusters: Vec<(usize, String)> = zones
                .iter()
                .map(|kv| (kv.1.len(), kv.0.to_owned()))
                .collect();
            clusters.sort();
            if clusters.len() > 2 && (clusters[0].0 + clusters[1].0 < 50) {
                let mut head = zones.remove(&clusters[0].1).unwrap();
                let mut tail = zones.remove(&clusters[1].1).unwrap();
                head.append(&mut tail);
                let key = format!("{}/{}", clusters[0].1, clusters[1].1);
                assert_eq!(zones.insert(key, head), None); // Should not collide
                continue;
            }
            let last = clusters.len() - 1;
            if clusters[last].0 > 100 {
                let mut to_split = zones.remove(&clusters[last].1).unwrap();
                to_split.sort();
                let second = to_split.split_off(clusters[last].0 / 2);
                let mut sub_idx = 1;
                loop {
                    let k1 = format!("{}_{}", clusters[last].1, sub_idx);
                    sub_idx += 1;
                    if !zones.contains_key(&k1) {
                        zones.insert(k1, to_split);
                        break;
                    }
                }
                loop {
                    let k2 = format!("{}_{}", clusters[last].1, sub_idx);
                    sub_idx += 1;
                    if !zones.contains_key(&k2) {
                        zones.insert(k2, second);
                        break;
                    }
                }
                continue;
            }
            break;
        }
        let mut keys: Vec<String> = zones.keys().map(|s| s.to_owned()).collect();
        keys.sort();
        for (i, k) in keys.iter().enumerate() {
            let mut chans = zones.remove(k).unwrap();
            chans.sort();
            let chan_a = chans[0].clone();
            let chan_b = chans[chans.len() - 1].clone();
            let clist = chans.join("|");
            let ent = format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\r\n",
                i + 1,
                k,
                clist,
                chan_a,
                chan_b
            );
            file.write_all(ent.as_bytes())?;
        }
    }
    Ok(())
}

// Main (entry point) - parse args and perform requested operations
fn main() {
    let matches = App::new("DJ-MD5 csv translator")
        .version("0.1")
        .author("Frank A. Stevenson")
        .about("Early days")
        .args_from_usage(
            "-c --input-contacts=[FILE]       'input contacts file'
                          -C --output-contacts=[FILE]      'output contacts file'
                          -f --input-channels=[FILE]       'input channels & frequence file'
                          -F --output-channels=[FILE]      'output channels ^ frequency file'
                          -G --output-groups=[FILE]        'output groups from channels to file'",
        )
        .get_matches();

    if let Some(input_contacts) = matches.value_of("input-contacts") {
        let contacts = read_contacts(&input_contacts);
        match contacts {
            Err(e) => {
                println!("failed reading contacts: {:?}", e);
            }
            Ok(c) => {
                if let Some(output_contacts) = matches.value_of("output-contacts") {
                    write_contacts(&output_contacts, &c).unwrap();
                }
            }
        }
    }

    if let Some(input_channels) = matches.value_of("input-channels") {
        let channels = read_channels(&input_channels);
        match channels {
            Err(e) => {
                println!("failed reading channels: {:?}", e);
            }
            Ok(c) => {
                if let Some(output_channels) = matches.value_of("output-channels") {
                    write_channels(&output_channels, &c, matches.value_of("output-groups"))
                        .unwrap();
                }
            }
        }
    }
}
