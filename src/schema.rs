// @generated automatically by Diesel CLI.

diesel::table! {
    agt_agent_types (agent_type_id) {
        agent_type_id -> Int4,
        #[max_length = 50]
        agent_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    agt_agents (agent_id) {
        agent_id -> Int4,
        division_id -> Nullable<Int4>,
        corporation_id -> Nullable<Int4>,
        location_id -> Nullable<Int4>,
        level -> Nullable<Int4>,
        quality -> Nullable<Int4>,
        agent_type_id -> Nullable<Int4>,
        is_locator -> Nullable<Bool>,
    }
}

diesel::table! {
    agt_agents_in_space (agent_id) {
        agent_id -> Int4,
        dungeon_id -> Nullable<Int4>,
        solar_system_id -> Nullable<Int4>,
        spawn_point_id -> Nullable<Int4>,
        type_id -> Nullable<Int4>,
    }
}

diesel::table! {
    agt_research_agents (agent_id, type_id) {
        agent_id -> Int4,
        type_id -> Int4,
    }
}

diesel::table! {
    cert_certs (cert_id) {
        cert_id -> Int4,
        description -> Nullable<Text>,
        group_id -> Nullable<Int4>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    cert_masteries (type_id, mastery_level, cert_id) {
        type_id -> Int4,
        mastery_level -> Int4,
        cert_id -> Int4,
    }
}

diesel::table! {
    cert_skills (cert_id, skill_id, cert_level_int, skill_level) {
        cert_id -> Int4,
        skill_id -> Int4,
        cert_level_int -> Int4,
        skill_level -> Int4,
        #[max_length = 8]
        cert_level_text -> Nullable<Varchar>,
    }
}

diesel::table! {
    chr_ancestries (ancestry_id) {
        ancestry_id -> Int4,
        #[max_length = 100]
        ancestry_name -> Nullable<Varchar>,
        bloodline_id -> Nullable<Int4>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        perception -> Nullable<Int4>,
        willpower -> Nullable<Int4>,
        charisma -> Nullable<Int4>,
        memory -> Nullable<Int4>,
        intelligence -> Nullable<Int4>,
        icon_id -> Nullable<Int4>,
        #[max_length = 500]
        short_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    chr_attributes (attribute_id) {
        attribute_id -> Int4,
        #[max_length = 100]
        attribute_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
        #[max_length = 500]
        short_description -> Nullable<Varchar>,
        #[max_length = 500]
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    chr_bloodlines (bloodline_id) {
        bloodline_id -> Int4,
        #[max_length = 100]
        bloodline_name -> Nullable<Varchar>,
        race_id -> Nullable<Int4>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        #[max_length = 1000]
        male_description -> Nullable<Varchar>,
        #[max_length = 1000]
        female_description -> Nullable<Varchar>,
        ship_type_id -> Nullable<Int4>,
        corporation_id -> Nullable<Int4>,
        perception -> Nullable<Int4>,
        willpower -> Nullable<Int4>,
        charisma -> Nullable<Int4>,
        memory -> Nullable<Int4>,
        intelligence -> Nullable<Int4>,
        icon_id -> Nullable<Int4>,
        #[max_length = 500]
        short_description -> Nullable<Varchar>,
        #[max_length = 500]
        short_male_description -> Nullable<Varchar>,
        #[max_length = 500]
        short_female_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    chr_factions (faction_id) {
        faction_id -> Int4,
        #[max_length = 100]
        faction_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        race_i_ds -> Nullable<Int4>,
        solar_system_id -> Nullable<Int4>,
        corporation_id -> Nullable<Int4>,
        size_factor -> Nullable<Float8>,
        station_count -> Nullable<Int4>,
        station_system_count -> Nullable<Int4>,
        militia_corporation_id -> Nullable<Int4>,
        icon_id -> Nullable<Int4>,
    }
}

diesel::table! {
    chr_races (race_id) {
        race_id -> Int4,
        #[max_length = 100]
        race_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
        #[max_length = 500]
        short_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    crp_activities (activity_id) {
        activity_id -> Int4,
        #[max_length = 100]
        activity_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    crp_npc_corporation_divisions (corporation_id, division_id) {
        corporation_id -> Int4,
        division_id -> Int4,
        size -> Nullable<Int4>,
    }
}

diesel::table! {
    crp_npc_corporation_research_fields (skill_id, corporation_id) {
        skill_id -> Int4,
        corporation_id -> Int4,
    }
}

diesel::table! {
    crp_npc_corporation_trades (corporation_id, type_id) {
        corporation_id -> Int4,
        type_id -> Int4,
    }
}

diesel::table! {
    crp_npc_corporations (corporation_id) {
        corporation_id -> Int4,
        #[max_length = 1]
        size -> Nullable<Bpchar>,
        #[max_length = 1]
        extent -> Nullable<Bpchar>,
        solar_system_id -> Nullable<Int4>,
        investor_id1 -> Nullable<Int4>,
        investor_shares1 -> Nullable<Int4>,
        investor_id2 -> Nullable<Int4>,
        investor_shares2 -> Nullable<Int4>,
        investor_id3 -> Nullable<Int4>,
        investor_shares3 -> Nullable<Int4>,
        investor_id4 -> Nullable<Int4>,
        investor_shares4 -> Nullable<Int4>,
        friend_id -> Nullable<Int4>,
        enemy_id -> Nullable<Int4>,
        public_shares -> Nullable<Int4>,
        initial_price -> Nullable<Int4>,
        min_security -> Nullable<Float8>,
        scattered -> Nullable<Bool>,
        fringe -> Nullable<Int4>,
        corridor -> Nullable<Int4>,
        hub -> Nullable<Int4>,
        border -> Nullable<Int4>,
        faction_id -> Nullable<Int4>,
        size_factor -> Nullable<Float8>,
        station_count -> Nullable<Int4>,
        station_system_count -> Nullable<Int4>,
        #[max_length = 4000]
        description -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
    }
}

diesel::table! {
    crp_npc_divisions (division_id) {
        division_id -> Int4,
        #[max_length = 100]
        division_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        #[max_length = 100]
        leader_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    dgm_attribute_categories (category_id) {
        category_id -> Int4,
        #[max_length = 50]
        category_name -> Nullable<Varchar>,
        #[max_length = 200]
        category_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    dgm_attribute_types (attribute_id) {
        attribute_id -> Int4,
        #[max_length = 100]
        attribute_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
        default_value -> Nullable<Float8>,
        published -> Nullable<Bool>,
        #[max_length = 150]
        display_name -> Nullable<Varchar>,
        unit_id -> Nullable<Int4>,
        stackable -> Nullable<Bool>,
        high_is_good -> Nullable<Bool>,
        category_id -> Nullable<Int4>,
    }
}

diesel::table! {
    dgm_effects (effect_id) {
        effect_id -> Int4,
        #[max_length = 400]
        effect_name -> Nullable<Varchar>,
        effect_category -> Nullable<Int4>,
        pre_expression -> Nullable<Int4>,
        post_expression -> Nullable<Int4>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        #[max_length = 60]
        guid -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
        is_offensive -> Nullable<Bool>,
        is_assistance -> Nullable<Bool>,
        duration_attribute_id -> Nullable<Int4>,
        tracking_speed_attribute_id -> Nullable<Int4>,
        discharge_attribute_id -> Nullable<Int4>,
        range_attribute_id -> Nullable<Int4>,
        falloff_attribute_id -> Nullable<Int4>,
        disallow_auto_repeat -> Nullable<Bool>,
        published -> Nullable<Bool>,
        #[max_length = 100]
        display_name -> Nullable<Varchar>,
        is_warp_safe -> Nullable<Bool>,
        range_chance -> Nullable<Bool>,
        electronic_chance -> Nullable<Bool>,
        propulsion_chance -> Nullable<Bool>,
        distribution -> Nullable<Int4>,
        #[max_length = 20]
        sfx_name -> Nullable<Varchar>,
        npc_usage_chance_attribute_id -> Nullable<Int4>,
        npc_activation_chance_attribute_id -> Nullable<Int4>,
        fitting_usage_chance_attribute_id -> Nullable<Int4>,
        modifier_info -> Nullable<Text>,
    }
}

diesel::table! {
    dgm_expressions (expression_id) {
        expression_id -> Int4,
        operand_id -> Nullable<Int4>,
        arg1 -> Nullable<Int4>,
        arg2 -> Nullable<Int4>,
        #[max_length = 100]
        expression_value -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        #[max_length = 500]
        expression_name -> Nullable<Varchar>,
        expression_type_id -> Nullable<Int4>,
        expression_group_id -> Nullable<Int4>,
        expression_attribute_id -> Nullable<Int4>,
    }
}

diesel::table! {
    dgm_type_attributes (type_id, attribute_id) {
        type_id -> Int4,
        attribute_id -> Int4,
        value_int -> Nullable<Int4>,
        value_float -> Nullable<Float8>,
    }
}

diesel::table! {
    dgm_type_effects (type_id, effect_id) {
        type_id -> Int4,
        effect_id -> Int4,
        is_default -> Nullable<Bool>,
    }
}

diesel::table! {
    eve_graphics (graphic_id) {
        graphic_id -> Int4,
        #[max_length = 100]
        sof_faction_name -> Nullable<Varchar>,
        #[max_length = 256]
        graphic_file -> Nullable<Varchar>,
        #[max_length = 100]
        sof_hull_name -> Nullable<Varchar>,
        #[max_length = 100]
        sof_race_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    eve_icons (icon_id) {
        icon_id -> Int4,
        #[max_length = 500]
        icon_file -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    eve_units (unit_id) {
        unit_id -> Int4,
        #[max_length = 100]
        unit_name -> Nullable<Varchar>,
        #[max_length = 50]
        display_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    industry_activity (type_id, activity_id) {
        type_id -> Int4,
        activity_id -> Int4,
        time -> Nullable<Int4>,
    }
}

diesel::table! {
    industry_activity_materials (type_id, activity_id, material_type_id) {
        type_id -> Int4,
        activity_id -> Int4,
        material_type_id -> Int4,
        quantity -> Nullable<Int4>,
    }
}

diesel::table! {
    industry_activity_probabilities (type_id, activity_id, product_type_id) {
        type_id -> Int4,
        activity_id -> Int4,
        product_type_id -> Int4,
        probability -> Nullable<Numeric>,
    }
}

diesel::table! {
    industry_activity_products (type_id, activity_id, product_type_id) {
        type_id -> Int4,
        activity_id -> Int4,
        product_type_id -> Int4,
        quantity -> Nullable<Int4>,
    }
}

diesel::table! {
    industry_activity_races (type_id, activity_id, product_type_id, race_id) {
        type_id -> Int4,
        activity_id -> Int4,
        product_type_id -> Int4,
        race_id -> Int4,
    }
}

diesel::table! {
    industry_activity_skills (id) {
        type_id -> Nullable<Int4>,
        activity_id -> Nullable<Int4>,
        skill_id -> Nullable<Int4>,
        level -> Nullable<Int4>,
        id -> Int4,
    }
}

diesel::table! {
    industry_blueprints (type_id) {
        type_id -> Int4,
        max_production_limit -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_categories (category_id) {
        category_id -> Int4,
        #[max_length = 100]
        category_name -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
        published -> Nullable<Bool>,
    }
}

diesel::table! {
    inv_contraband_types (faction_id, type_id) {
        faction_id -> Int4,
        type_id -> Int4,
        standing_loss -> Nullable<Float8>,
        confiscate_min_sec -> Nullable<Float8>,
        fine_by_value -> Nullable<Float8>,
        attack_min_sec -> Nullable<Float8>,
    }
}

diesel::table! {
    inv_control_tower_resource_purposes (purpose) {
        purpose -> Int4,
        #[max_length = 100]
        purpose_text -> Nullable<Varchar>,
    }
}

diesel::table! {
    inv_control_tower_resources (control_tower_type_id, resource_type_id) {
        control_tower_type_id -> Int4,
        resource_type_id -> Int4,
        purpose -> Nullable<Int4>,
        quantity -> Nullable<Int4>,
        min_security_level -> Nullable<Float8>,
        faction_id -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_flags (flag_id) {
        flag_id -> Int4,
        #[max_length = 200]
        flag_name -> Nullable<Varchar>,
        #[max_length = 100]
        flag_text -> Nullable<Varchar>,
        order_id -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_groups (group_id) {
        group_id -> Int4,
        category_id -> Nullable<Int4>,
        #[max_length = 100]
        group_name -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
        use_base_price -> Nullable<Bool>,
        anchored -> Nullable<Bool>,
        anchorable -> Nullable<Bool>,
        fittable_non_singleton -> Nullable<Bool>,
        published -> Nullable<Bool>,
    }
}

diesel::table! {
    inv_items (item_id) {
        item_id -> Int4,
        type_id -> Int4,
        owner_id -> Int4,
        location_id -> Int4,
        flag_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    inv_market_groups (market_group_id) {
        market_group_id -> Int4,
        parent_group_id -> Nullable<Int4>,
        #[max_length = 100]
        market_group_name -> Nullable<Varchar>,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
        has_types -> Nullable<Bool>,
    }
}

diesel::table! {
    inv_meta_groups (meta_group_id) {
        meta_group_id -> Int4,
        #[max_length = 100]
        meta_group_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        icon_id -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_meta_types (type_id) {
        type_id -> Int4,
        parent_type_id -> Nullable<Int4>,
        meta_group_id -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_names (item_id) {
        item_id -> Int4,
        #[max_length = 200]
        item_name -> Varchar,
    }
}

diesel::table! {
    inv_positions (item_id) {
        item_id -> Int4,
        x -> Float8,
        y -> Float8,
        z -> Float8,
        yaw -> Nullable<Float4>,
        pitch -> Nullable<Float4>,
        roll -> Nullable<Float4>,
    }
}

diesel::table! {
    inv_traits (trait_id) {
        trait_id -> Int4,
        type_id -> Nullable<Int4>,
        skill_id -> Nullable<Int4>,
        bonus -> Nullable<Float8>,
        bonus_text -> Nullable<Text>,
        unit_id -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_type_materials (type_id, material_type_id) {
        type_id -> Int4,
        material_type_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    inv_type_reactions (reaction_type_id, input, type_id) {
        reaction_type_id -> Int4,
        input -> Bool,
        type_id -> Int4,
        quantity -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_types (type_id) {
        type_id -> Int4,
        group_id -> Nullable<Int4>,
        #[max_length = 100]
        type_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        mass -> Nullable<Float8>,
        volume -> Nullable<Float8>,
        capacity -> Nullable<Float8>,
        portion_size -> Nullable<Int4>,
        race_id -> Nullable<Int4>,
        base_price -> Nullable<Numeric>,
        published -> Nullable<Bool>,
        market_group_id -> Nullable<Int4>,
        icon_id -> Nullable<Int4>,
        sound_id -> Nullable<Int4>,
        graphic_id -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_unique_names (item_id) {
        item_id -> Int4,
        #[max_length = 200]
        item_name -> Varchar,
        group_id -> Nullable<Int4>,
    }
}

diesel::table! {
    inv_volumes (type_id) {
        type_id -> Int4,
        volume -> Nullable<Int4>,
    }
}

diesel::table! {
    map_celestial_graphics (celestial_id) {
        celestial_id -> Int4,
        height_map1 -> Nullable<Int4>,
        height_map2 -> Nullable<Int4>,
        shader_preset -> Nullable<Int4>,
        population -> Nullable<Bool>,
    }
}

diesel::table! {
    map_celestial_statistics (celestial_id) {
        celestial_id -> Int4,
        temperature -> Nullable<Float8>,
        #[max_length = 10]
        spectral_class -> Nullable<Varchar>,
        luminosity -> Nullable<Float8>,
        age -> Nullable<Float8>,
        life -> Nullable<Float8>,
        orbit_radius -> Nullable<Float8>,
        eccentricity -> Nullable<Float8>,
        mass_dust -> Nullable<Float8>,
        mass_gas -> Nullable<Float8>,
        fragmented -> Nullable<Bool>,
        density -> Nullable<Float8>,
        surface_gravity -> Nullable<Float8>,
        escape_velocity -> Nullable<Float8>,
        orbit_period -> Nullable<Float8>,
        rotation_rate -> Nullable<Float8>,
        locked -> Nullable<Bool>,
        pressure -> Nullable<Float8>,
        radius -> Nullable<Float8>,
        mass -> Nullable<Int4>,
    }
}

diesel::table! {
    map_constellation_jumps (from_constellation_id, to_constellation_id) {
        from_region_id -> Nullable<Int4>,
        from_constellation_id -> Int4,
        to_constellation_id -> Int4,
        to_region_id -> Nullable<Int4>,
    }
}

diesel::table! {
    map_constellations (constellation_id) {
        region_id -> Nullable<Int4>,
        constellation_id -> Int4,
        #[max_length = 100]
        constellation_name -> Nullable<Varchar>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        x_min -> Nullable<Float8>,
        x_max -> Nullable<Float8>,
        y_min -> Nullable<Float8>,
        y_max -> Nullable<Float8>,
        z_min -> Nullable<Float8>,
        z_max -> Nullable<Float8>,
        faction_id -> Nullable<Int4>,
        radius -> Nullable<Float8>,
    }
}

diesel::table! {
    map_denormalize (item_id) {
        item_id -> Int4,
        type_id -> Nullable<Int4>,
        group_id -> Nullable<Int4>,
        solar_system_id -> Nullable<Int4>,
        constellation_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        orbit_id -> Nullable<Int4>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        radius -> Nullable<Float8>,
        #[max_length = 100]
        item_name -> Nullable<Varchar>,
        security -> Nullable<Float8>,
        celestial_index -> Nullable<Int4>,
        orbit_index -> Nullable<Int4>,
    }
}

diesel::table! {
    map_jumps (stargate_id) {
        stargate_id -> Int4,
        destination_id -> Nullable<Int4>,
    }
}

diesel::table! {
    map_landmarks (landmark_id) {
        landmark_id -> Int4,
        #[max_length = 100]
        landmark_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        location_id -> Nullable<Int4>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        icon_id -> Nullable<Int4>,
    }
}

diesel::table! {
    map_location_scenes (location_id) {
        location_id -> Int4,
        graphic_id -> Nullable<Int4>,
    }
}

diesel::table! {
    map_location_wormhole_classes (location_id) {
        location_id -> Int4,
        wormhole_class_id -> Nullable<Int4>,
    }
}

diesel::table! {
    map_region_jumps (from_region_id, to_region_id) {
        from_region_id -> Int4,
        to_region_id -> Int4,
    }
}

diesel::table! {
    map_regions (region_id) {
        region_id -> Int4,
        #[max_length = 100]
        region_name -> Nullable<Varchar>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        x_min -> Nullable<Float8>,
        x_max -> Nullable<Float8>,
        y_min -> Nullable<Float8>,
        y_max -> Nullable<Float8>,
        z_min -> Nullable<Float8>,
        z_max -> Nullable<Float8>,
        faction_id -> Nullable<Int4>,
        nebula -> Nullable<Int4>,
        radius -> Nullable<Float8>,
    }
}

diesel::table! {
    map_solar_system_jumps (from_solar_system_id, to_solar_system_id) {
        from_region_id -> Nullable<Int4>,
        from_constellation_id -> Nullable<Int4>,
        from_solar_system_id -> Int4,
        to_solar_system_id -> Int4,
        to_constellation_id -> Nullable<Int4>,
        to_region_id -> Nullable<Int4>,
    }
}

diesel::table! {
    map_solar_systems (solar_system_id) {
        region_id -> Nullable<Int4>,
        constellation_id -> Nullable<Int4>,
        solar_system_id -> Int4,
        #[max_length = 100]
        solar_system_name -> Nullable<Varchar>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        x_min -> Nullable<Float8>,
        x_max -> Nullable<Float8>,
        y_min -> Nullable<Float8>,
        y_max -> Nullable<Float8>,
        z_min -> Nullable<Float8>,
        z_max -> Nullable<Float8>,
        luminosity -> Nullable<Float8>,
        border -> Nullable<Bool>,
        fringe -> Nullable<Bool>,
        corridor -> Nullable<Bool>,
        hub -> Nullable<Bool>,
        international -> Nullable<Bool>,
        regional -> Nullable<Bool>,
        constellation -> Nullable<Bool>,
        security -> Nullable<Float8>,
        faction_id -> Nullable<Int4>,
        radius -> Nullable<Float8>,
        sun_type_id -> Nullable<Int4>,
        #[max_length = 2]
        security_class -> Nullable<Varchar>,
    }
}

diesel::table! {
    map_universe (universe_id) {
        universe_id -> Int4,
        #[max_length = 100]
        universe_name -> Nullable<Varchar>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        x_min -> Nullable<Float8>,
        x_max -> Nullable<Float8>,
        y_min -> Nullable<Float8>,
        y_max -> Nullable<Float8>,
        z_min -> Nullable<Float8>,
        z_max -> Nullable<Float8>,
        radius -> Nullable<Float8>,
    }
}

diesel::table! {
    planet_schematics (schematic_id) {
        schematic_id -> Int4,
        #[max_length = 255]
        schematic_name -> Nullable<Varchar>,
        cycle_time -> Nullable<Int4>,
    }
}

diesel::table! {
    planet_schematics_pin_map (schematic_id, pin_type_id) {
        schematic_id -> Int4,
        pin_type_id -> Int4,
    }
}

diesel::table! {
    planet_schematics_type_map (schematic_id, type_id) {
        schematic_id -> Int4,
        type_id -> Int4,
        quantity -> Nullable<Int4>,
        is_input -> Nullable<Bool>,
    }
}

diesel::table! {
    ram_activities (activity_id) {
        activity_id -> Int4,
        #[max_length = 100]
        activity_name -> Nullable<Varchar>,
        #[max_length = 5]
        icon_no -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        published -> Nullable<Bool>,
    }
}

diesel::table! {
    ram_assembly_line_stations (station_id, assembly_line_type_id) {
        station_id -> Int4,
        assembly_line_type_id -> Int4,
        quantity -> Nullable<Int4>,
        station_type_id -> Nullable<Int4>,
        owner_id -> Nullable<Int4>,
        solar_system_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
    }
}

diesel::table! {
    ram_assembly_line_type_detail_per_category (assembly_line_type_id, category_id) {
        assembly_line_type_id -> Int4,
        category_id -> Int4,
        time_multiplier -> Nullable<Float8>,
        material_multiplier -> Nullable<Float8>,
        cost_multiplier -> Nullable<Float8>,
    }
}

diesel::table! {
    ram_assembly_line_type_detail_per_group (assembly_line_type_id, group_id) {
        assembly_line_type_id -> Int4,
        group_id -> Int4,
        time_multiplier -> Nullable<Float8>,
        material_multiplier -> Nullable<Float8>,
        cost_multiplier -> Nullable<Float8>,
    }
}

diesel::table! {
    ram_assembly_line_types (assembly_line_type_id) {
        assembly_line_type_id -> Int4,
        #[max_length = 100]
        assembly_line_type_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        base_time_multiplier -> Nullable<Float8>,
        base_material_multiplier -> Nullable<Float8>,
        base_cost_multiplier -> Nullable<Float8>,
        volume -> Nullable<Float8>,
        activity_id -> Nullable<Int4>,
        min_cost_per_hour -> Nullable<Float8>,
    }
}

diesel::table! {
    ram_installation_type_contents (installation_type_id, assembly_line_type_id) {
        installation_type_id -> Int4,
        assembly_line_type_id -> Int4,
        quantity -> Nullable<Int4>,
    }
}

diesel::table! {
    skin_license (license_type_id) {
        license_type_id -> Int4,
        duration -> Nullable<Int4>,
        skin_id -> Nullable<Int4>,
    }
}

diesel::table! {
    skin_materials (skin_material_id) {
        skin_material_id -> Int4,
        display_name_id -> Nullable<Int4>,
        material_set_id -> Nullable<Int4>,
    }
}

diesel::table! {
    skin_ship (skin_id, type_id) {
        skin_id -> Int4,
        type_id -> Int4,
    }
}

diesel::table! {
    skins (skin_id) {
        skin_id -> Int4,
        #[max_length = 70]
        internal_name -> Nullable<Varchar>,
        skin_material_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sta_operation_services (operation_id, service_id) {
        operation_id -> Int4,
        service_id -> Int4,
    }
}

diesel::table! {
    sta_operations (operation_id) {
        activity_id -> Nullable<Int4>,
        operation_id -> Int4,
        #[max_length = 100]
        operation_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        fringe -> Nullable<Int4>,
        corridor -> Nullable<Int4>,
        hub -> Nullable<Int4>,
        border -> Nullable<Int4>,
        ratio -> Nullable<Int4>,
        caldari_station_type_id -> Nullable<Int4>,
        minmatar_station_type_id -> Nullable<Int4>,
        amarr_station_type_id -> Nullable<Int4>,
        gallente_station_type_id -> Nullable<Int4>,
        jove_station_type_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sta_services (service_id) {
        service_id -> Int4,
        #[max_length = 100]
        service_name -> Nullable<Varchar>,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    sta_station_types (station_type_id) {
        station_type_id -> Int4,
        dock_entry_x -> Nullable<Float8>,
        dock_entry_y -> Nullable<Float8>,
        dock_entry_z -> Nullable<Float8>,
        dock_orientation_x -> Nullable<Float8>,
        dock_orientation_y -> Nullable<Float8>,
        dock_orientation_z -> Nullable<Float8>,
        operation_id -> Nullable<Int4>,
        office_slots -> Nullable<Int4>,
        reprocessing_efficiency -> Nullable<Float8>,
        conquerable -> Nullable<Bool>,
    }
}

diesel::table! {
    sta_stations (station_id) {
        station_id -> Int8,
        security -> Nullable<Float8>,
        docking_cost_per_volume -> Nullable<Float8>,
        max_ship_volume_dockable -> Nullable<Float8>,
        office_rental_cost -> Nullable<Int4>,
        operation_id -> Nullable<Int4>,
        station_type_id -> Nullable<Int4>,
        corporation_id -> Nullable<Int4>,
        solar_system_id -> Nullable<Int4>,
        constellation_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        #[max_length = 100]
        station_name -> Nullable<Varchar>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        reprocessing_efficiency -> Nullable<Float8>,
        reprocessing_stations_take -> Nullable<Float8>,
        reprocessing_hangar_flag -> Nullable<Int4>,
    }
}

diesel::table! {
    translation_tables (source_table, translated_key) {
        #[max_length = 200]
        source_table -> Varchar,
        #[max_length = 200]
        destination_table -> Nullable<Varchar>,
        #[max_length = 200]
        translated_key -> Varchar,
        tc_group_id -> Nullable<Int4>,
        tc_id -> Nullable<Int4>,
    }
}

diesel::table! {
    trn_translation_columns (tc_id) {
        tc_group_id -> Nullable<Int4>,
        tc_id -> Int4,
        #[max_length = 256]
        table_name -> Varchar,
        #[max_length = 128]
        column_name -> Varchar,
        #[max_length = 128]
        master_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    trn_translation_languages (numeric_language_id) {
        numeric_language_id -> Int4,
        #[max_length = 50]
        language_id -> Nullable<Varchar>,
        #[max_length = 200]
        language_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    trn_translations (tc_id, key_id, language_id) {
        tc_id -> Int4,
        key_id -> Int4,
        #[max_length = 50]
        language_id -> Varchar,
        text -> Text,
    }
}

diesel::table! {
    war_combat_zone_systems (solar_system_id) {
        solar_system_id -> Int4,
        combat_zone_id -> Nullable<Int4>,
    }
}

diesel::table! {
    war_combat_zones (combat_zone_id) {
        combat_zone_id -> Int4,
        #[max_length = 100]
        combat_zone_name -> Nullable<Varchar>,
        faction_id -> Nullable<Int4>,
        center_system_id -> Nullable<Int4>,
        #[max_length = 500]
        description -> Nullable<Varchar>,
    }
}

diesel::joinable!(inv_type_materials -> inv_types (type_id));

diesel::allow_tables_to_appear_in_same_query!(
    agt_agent_types,
    agt_agents,
    agt_agents_in_space,
    agt_research_agents,
    cert_certs,
    cert_masteries,
    cert_skills,
    chr_ancestries,
    chr_attributes,
    chr_bloodlines,
    chr_factions,
    chr_races,
    crp_activities,
    crp_npc_corporation_divisions,
    crp_npc_corporation_research_fields,
    crp_npc_corporation_trades,
    crp_npc_corporations,
    crp_npc_divisions,
    dgm_attribute_categories,
    dgm_attribute_types,
    dgm_effects,
    dgm_expressions,
    dgm_type_attributes,
    dgm_type_effects,
    eve_graphics,
    eve_icons,
    eve_units,
    industry_activity,
    industry_activity_materials,
    industry_activity_probabilities,
    industry_activity_products,
    industry_activity_races,
    industry_activity_skills,
    industry_blueprints,
    inv_categories,
    inv_contraband_types,
    inv_control_tower_resource_purposes,
    inv_control_tower_resources,
    inv_flags,
    inv_groups,
    inv_items,
    inv_market_groups,
    inv_meta_groups,
    inv_meta_types,
    inv_names,
    inv_positions,
    inv_traits,
    inv_type_materials,
    inv_type_reactions,
    inv_types,
    inv_unique_names,
    inv_volumes,
    map_celestial_graphics,
    map_celestial_statistics,
    map_constellation_jumps,
    map_constellations,
    map_denormalize,
    map_jumps,
    map_landmarks,
    map_location_scenes,
    map_location_wormhole_classes,
    map_region_jumps,
    map_regions,
    map_solar_system_jumps,
    map_solar_systems,
    map_universe,
    planet_schematics,
    planet_schematics_pin_map,
    planet_schematics_type_map,
    ram_activities,
    ram_assembly_line_stations,
    ram_assembly_line_type_detail_per_category,
    ram_assembly_line_type_detail_per_group,
    ram_assembly_line_types,
    ram_installation_type_contents,
    skin_license,
    skin_materials,
    skin_ship,
    skins,
    sta_operation_services,
    sta_operations,
    sta_services,
    sta_station_types,
    sta_stations,
    translation_tables,
    trn_translation_columns,
    trn_translation_languages,
    trn_translations,
    war_combat_zone_systems,
    war_combat_zones,
);
