#!/usr/bin/env python3

import sys
from pathlib import Path
import json

def update_metadata_image_urls(images_cid: str):
    """Update all metadata files with the correct image CIDs."""
    metadata_dir = Path("ipfs/metadata")
    
    if not metadata_dir.exists():
        print("❌ Error: ipfs/metadata directory not found")
        return False
    
    try:
        for metadata_file in sorted(metadata_dir.glob("*.json"), key=lambda x: int(x.stem)):
            with open(metadata_file) as f:
                metadata = json.load(f)
            
            # Update image URL with actual CID
            metadata["image"] = metadata["image"].replace("IMAGES_CID", images_cid)
            
            with open(metadata_file, "w") as f:
                json.dump(metadata, f, indent=2)
            print(f"✅ Updated {metadata_file.name}")
        
        print(f"\n✨ Successfully updated all metadata files with images CID: {images_cid}")
        print("\nNext step:")
        print("Upload metadata:")
        print("pinata-web3 upload ipfs/metadata")
        return True
    except Exception as e:
        print(f"❌ Error updating metadata: {str(e)}")
        return False

def main():
    if len(sys.argv) != 2:
        print("Usage: python update_metadata.py <IMAGES_CID>")
        print("Example: python update_metadata.py QmYourImagesCID123")
        sys.exit(1)
    
    images_cid = sys.argv[1]
    if not update_metadata_image_urls(images_cid):
        sys.exit(1)

if __name__ == "__main__":
    main() 