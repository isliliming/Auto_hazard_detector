use pyo3::prelude::*;

/// Looks up the hazard level for a given H-code.
/// Returns "High", "Medium", "Low", or "Unknown".
#[pyfunction]
fn get_hazard_level(h_code: String) -> String {
    match h_code.as_str() {
        // --- High Hazard ---
        // Fatal/Toxic
        "H300" | "H301" | "H310" | "H311" | "H330" | "H331" => "High".to_string(),
        // Severe damage
        "H314" | "H318" => "High".to_string(),
        // CMR (Carcinogenic, Mutagenic, Reprotoxic) Category 1A/1B
        "H340" | "H350" | "H360" => "High".to_string(),
        // STOT SE 1 / RE 1
        "H370" | "H372" => "High".to_string(),
        // Aspiration
        "H304" => "High".to_string(),
        // Respiratory sensitization
        "H334" => "High".to_string(),
        // Explosives (Category 1.1, 1.2, 1.3)
        "H200" | "H201" | "H202" | "H203" => "High".to_string(),

        // --- Medium Hazard ---
        // Harmful
        "H302" | "H312" | "H332" => "Medium".to_string(),
        // Irritation (Eye) / Sensitization (Skin)
        "H319" | "H317" => "Medium".to_string(),
        // CMR Category 2
        "H341" | "H351" | "H361" => "Medium".to_string(),
        // STOT SE 2 / RE 2
        "H371" | "H373" => "Medium".to_string(),
        // Environmental (Acute 1, Chronic 1)
        "H400" | "H410" => "Medium".to_string(),

        // --- Low Hazard ---
        // May be harmful
        "H303" | "H313" | "H333" => "Low".to_string(),
        // Skin Irritation
        "H315" => "Low".to_string(),
        // STOT SE 3 (Respiratory/Narcotic)
        "H335" | "H336" => "Low".to_string(),
        // Environmental (Chronic 2, 3, 4)
        "H411" | "H412" | "H413" => "Low".to_string(),

        // --- Default ---
        _ => "Unknown".to_string(),
    }
}

/// Takes a list of H-codes and returns the highest hazard level found.
#[pyfunction]
fn assess_hazards(h_codes: Vec<String>) -> String {
    let mut max_hazard = 0; // 0: Unknown, 1: Low, 2: Medium, 3: High

    for code in h_codes {
        let level = get_hazard_level(code);
        let current_hazard = match level.as_str() {
            "High" => 3,
            "Medium" => 2,
            "Low" => 1,
            _ => 0,
        };
        if current_hazard > max_hazard {
            max_hazard = current_hazard;
        }
    }

    match max_hazard {
        3 => "High".to_string(),
        2 => "Medium".to_string(),
        1 => "Low".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[pymodule]
fn web_hazard_project(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_hazard_level, m)?)?;
    m.add_function(wrap_pyfunction!(assess_hazards, m)?)?;
    Ok(())
}
