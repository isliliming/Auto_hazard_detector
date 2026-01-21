import requests
import re
import web_hazard_project

def get_h_codes_recursive(cas_number):
    # Step 1: Get CID
    cid_url = f"https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{cas_number}/cids/JSON"
    try:
        cid_resp = requests.get(cid_url)
        cid_resp.raise_for_status()
        cid = cid_resp.json()['IdentifierList']['CID'][0]
        print(f"Found CID: {cid}")
    except Exception as e:
        return f"Error getting CID: {e}"

    # Step 2: Get the full 'Safety and Hazards' section (Broader search)
    # We use 'Safety+and+Hazards' heading to ensure we catch GHS data even if it's nested differently
    url = f"https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound/{cid}/JSON?heading=Safety+and+Hazards"

    try:
        resp = requests.get(url)
        # If Safety section is missing, try 'Hazards Identification' or generic
        if resp.status_code == 404:
            print("Specific heading not found, trying full record (slower)...")
            url = f"https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound/{cid}/JSON"
            resp = requests.get(url)

        resp.raise_for_status()
        data = resp.json()
    except Exception as e:
        return f"Error getting data: {e}"

    # Step 3: Recursive Search Function
    # This finds 'H3xx' codes anywhere in the JSON structure
    h_codes = set()

    def find_h_codes(obj):
        if isinstance(obj, dict):
            # Check if this node is a GHS Hazard Statement
            if obj.get('Name') == 'GHS Hazard Statements':
                # Extract text from the Value list
                for item in obj.get('Value', {}).get('StringWithMarkup', []):
                    text = item.get('String', '')
                    # Regex for H-code (H followed by 3 digits)
                    match = re.search(r"(H\d{3}[a-zA-Z]*)", text)
                    if match:
                        h_codes.add(match.group(1))

            # Recursive dive into all keys
            for key, value in obj.items():
                find_h_codes(value)

        elif isinstance(obj, list):
            # Recursive dive into list items
            for item in obj:
                find_h_codes(item)

    find_h_codes(data)
    return list(sorted(h_codes))

# --- Run ---
if __name__ == "__main__":
    cas = '60628-96-8'
    codes = get_h_codes_recursive(cas)
    print(f"H-Codes for {cas}: {codes}")

    if isinstance(codes, list):
        # Call the Rust library to assess hazards
        level = web_hazard_project.assess_hazards(codes)
        print(f"Overall Hazard Level: {level}")
    else:
        print("Could not assess hazards due to error in fetching codes.")
