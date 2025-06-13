let start_cue_list_id = 601;
let groups = [3, 4, 6, 7, 8];

for group in groups {
    record_color_cuelists(group, 1..13, start_cue_list_id);
    start_cue_list_id += 16;
}

fn record_color_cuelists(
    group,
    presets,
    cuelist_start,
) {
    for (i, preset) in (presets.start..=presets.end).enumerate() {
        let cuelist_id = cuelist_start + i as u16;
        conn.select_group(group);
        conn.select_preset(PresetType::Color, preset);
        conn.record(Key::CueList, cuelist_id, RecordMode::Replace);
        conn.enter();
        conn.clear();
    }
}
