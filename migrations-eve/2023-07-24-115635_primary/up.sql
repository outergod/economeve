-- Your SQL goes here

ALTER TABLE cert_masteries ADD PRIMARY KEY ("type_id", "mastery_level", "cert_id");
ALTER TABLE cert_skills ADD PRIMARY KEY ("cert_id", "skill_id", "cert_level_int", "skill_level");
ALTER TABLE industry_activity_materials ADD PRIMARY KEY ("type_id", "activity_id", "material_type_id");
ALTER TABLE industry_activity_probabilities ADD PRIMARY KEY ("type_id", "activity_id", "product_type_id");
ALTER TABLE industry_activity_products ADD PRIMARY KEY ("type_id", "activity_id", "product_type_id");
ALTER TABLE industry_activity_races ADD PRIMARY KEY ("type_id", "activity_id", "product_type_id", "race_id");
ALTER TABLE skin_ship ADD PRIMARY KEY ("skin_id", "type_id");
ALTER TABLE industry_activity_skills ADD COLUMN id SERIAL PRIMARY KEY;
