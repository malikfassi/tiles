#!/usr/bin/env python3

import os
import requests
import json
from pathlib import Path
from dotenv import load_dotenv
import shutil
import asyncio
import aiohttp
import logging
from typing import Dict, Tuple
from datetime import datetime

# Set up logging
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(),
        logging.FileHandler('upload.log')
    ]
)
logger = logging.getLogger(__name__)

# Load environment variables from .env file
load_dotenv()

class PinataAPI:
    def __init__(self, api_key: str, api_secret: str):
        self.api_key = api_key
        self.api_secret = api_secret
        self.headers = {
            "pinata_api_key": api_key,
            "pinata_secret_api_key": api_secret,
            "Content-Type": "application/json"
        }
        self.gateway_url = "https://gateway.pinata.cloud/ipfs"
    
    async def pin_json_async(self, session: aiohttp.ClientSession, json_data: dict, name: str = None) -> str:
        """Pin JSON data to IPFS asynchronously."""
        url = "https://api.pinata.cloud/pinning/pinJSONToIPFS"
        
        body = {
            "pinataContent": json_data
        }
        
        if name:
            body["pinataMetadata"] = {"name": name}
        
        start_time = datetime.now()
        logger.info(f"Starting JSON upload for {name}")
        
        try:
            async with session.post(url, json=body, headers=self.headers) as response:
                if response.status == 200:
                    result = await response.json()
                    duration = (datetime.now() - start_time).total_seconds()
                    logger.info(f"✅ JSON upload successful for {name} - CID: {result['IpfsHash']} (took {duration:.2f}s)")
                    return result["IpfsHash"]
                else:
                    error_text = await response.text()
                    logger.error(f"❌ JSON upload failed for {name}: {error_text}")
                    return None
        except Exception as e:
            logger.error(f"❌ Exception during JSON upload for {name}: {str(e)}")
            return None

    async def pin_file_async(self, session: aiohttp.ClientSession, file_path: str, name: str = None) -> Tuple[str, str]:
        """Pin a single file to IPFS asynchronously."""
        url = "https://api.pinata.cloud/pinning/pinFileToIPFS"
        file_name = Path(file_path).name
        
        start_time = datetime.now()
        logger.info(f"Starting file upload for {file_name}")
        
        try:
            # Read file content
            with open(file_path, 'rb') as f:
                file_content = f.read()
            
            # Prepare form data
            form = aiohttp.FormData()
            form.add_field('file', 
                          file_content,
                          filename=file_name,
                          content_type='application/octet-stream')
            
            if name:
                form.add_field('pinataMetadata', json.dumps({"name": name}))
            
            headers = {
                "pinata_api_key": self.api_key,
                "pinata_secret_api_key": self.api_secret
            }
            
            async with session.post(url, data=form, headers=headers) as response:
                if response.status == 200:
                    result = await response.json()
                    duration = (datetime.now() - start_time).total_seconds()
                    logger.info(f"✅ File upload successful for {file_name} - CID: {result['IpfsHash']} (took {duration:.2f}s)")
                    return (Path(file_path).stem, result["IpfsHash"])
                else:
                    error_text = await response.text()
                    logger.error(f"❌ File upload failed for {file_name}: {error_text}")
                    return (Path(file_path).stem, None)
        except Exception as e:
            logger.error(f"❌ Exception during file upload for {file_name}: {str(e)}")
            return (Path(file_path).stem, None)

    def pin_file(self, file_path: str, name: str = None) -> str:
        """Pin a single file to IPFS."""
        url = "https://api.pinata.cloud/pinning/pinFileToIPFS"
        
        files = [
            ('file', (
                Path(file_path).name,
                open(file_path, 'rb'),
                'application/octet-stream'
            ))
        ]
        
        metadata = {}
        if name:
            metadata = {"name": name}
        
        response = requests.post(
            url,
            files=files,
            data={'pinataMetadata': json.dumps(metadata)} if metadata else None,
            headers={"pinata_api_key": self.api_key, "pinata_secret_api_key": self.api_secret}
        )
        
        if response.status_code == 200:
            return response.json()["IpfsHash"]
        else:
            print(f"❌ File upload failed: {response.text}")
            return None

    async def pin_directory(self, dir_path: str) -> str:
        """Pin an entire directory to IPFS and return its CID"""
        url = "https://api.pinata.cloud/pinning/pinFileToIPFS"
        
        if not os.path.isdir(dir_path):
            raise ValueError(f"Directory not found: {dir_path}")
            
        # Create form data
        form = aiohttp.FormData()
        
        # Add all files in directory recursively
        for root, _, filenames in os.walk(dir_path):
            for filename in filenames:
                file_path = os.path.join(root, filename)
                relative_path = os.path.relpath(file_path, dir_path)
                
                # Read file content
                with open(file_path, 'rb') as f:
                    file_content = f.read()
                
                # Add file to form with the correct field name
                form.add_field(
                    'file',  # Must be 'file' for each file
                    file_content,
                    filename=relative_path  # Use relative path as filename
                )
        
        headers = {
            "pinata_api_key": self.api_key,
            "pinata_secret_api_key": self.api_secret
        }
        
        # Upload entire directory in a single request
        async with aiohttp.ClientSession() as session:
            async with session.post(url, data=form, headers=headers) as response:
                if response.status == 200:
                    result = await response.json()
                    logger.info(f"✅ Directory upload successful - CID: {result['IpfsHash']}")
                    return result['IpfsHash']
                else:
                    error_text = await response.text()
                    logger.error(f"❌ Directory upload failed: {error_text}")
                    raise Exception(f"Failed to pin directory: {error_text}")

    def replace_pin(self, old_cid: str, new_json: dict) -> str:
        """Replace an existing pin with new content."""
        url = "https://api.pinata.cloud/pinning/replacePinFromHash"
        
        body = {
            "ipfs_pin_hash": old_cid,
            "pinataContent": new_json
        }
        
        response = requests.post(url, json=body, headers=self.headers)
        
        if response.status_code == 200:
            return response.json()["IpfsHash"]
        else:
            print(f"❌ Replace failed: {response.text}")
            return None
    
    def get_pin_json(self, cid: str) -> dict:
        """Fetch JSON content from IPFS."""
        url = f"{self.gateway_url}/{cid}"
        response = requests.get(url)
        
        if response.status_code == 200:
            return response.json()
        else:
            print(f"❌ Fetch failed: {response.text}")
            return None

    async def get_all_metadata_async(self, token_ids: list) -> dict:
        """Fetch metadata for multiple tokens asynchronously."""
        import aiohttp
        import asyncio
        
        async def fetch_metadata(session, token_id):
            url = f"{self.gateway_url}/{token_id}"
            async with session.get(url) as response:
                if response.status == 200:
                    return token_id, await response.json()
                return token_id, None

        async with aiohttp.ClientSession() as session:
            tasks = [fetch_metadata(session, tid) for tid in token_ids]
            results = await asyncio.gather(*tasks)
            return dict(results)

def prepare_directory_structure():
    """Prepare the IPFS directory structure for Stargaze compatibility."""
    try:
        # Create directories with Stargaze-compatible structure
        Path("ipfs/images").mkdir(parents=True, exist_ok=True)
        Path("ipfs/metadata").mkdir(parents=True, exist_ok=True)
        
        # Move logo file if it exists
        logo_src = Path("ipfs/logo.png")
        if logo_src.exists():
            logger.info("Found logo file")
            # Keep logo at root level as per Stargaze structure
            
        # Get all files except logo
        png_files = [f for f in Path("ipfs").glob("*.png") if f.stem != "logo"]
        json_files = list(Path("ipfs").glob("*.json"))
        
        # Sort files by numerical order
        png_files.sort(key=lambda x: int(x.stem))
        json_files.sort(key=lambda x: int(x.stem))
        
        # Verify we have matching numbers of files
        if len(png_files) != len(json_files):
            logger.error("❌ Number of PNG files does not match number of JSON files")
            logger.error(f"Found {len(png_files)} PNG files and {len(json_files)} JSON files")
            return False
        
        # Move and rename files to match Stargaze structure
        for i, (png_file, json_file) in enumerate(zip(png_files, json_files), start=1):
            # Move to correct directories with sequential naming
            shutil.move(str(png_file), f"ipfs/images/{i}.png")
            
            # Read and update metadata to ensure correct image path
            with open(json_file) as f:
                metadata = json.load(f)
            
            # Save metadata with correct name in metadata directory
            with open(f"ipfs/metadata/{i}.json", "w") as f:
                json.dump(metadata, f, indent=2)
            
            # Remove original json file
            json_file.unlink()
            
            logger.info(f"Processed file pair {i}: {png_file.name} -> images/{i}.png, {json_file.name} -> metadata/{i}.json")
        
        logger.info("✅ Directory structure prepared according to Stargaze requirements")
        return True
        
    except Exception as e:
        logger.error(f"❌ Failed to prepare directory structure: {str(e)}")
        return False

async def verify_stargaze_compatibility(pinata: PinataAPI, image_cids: Dict[str, str], metadata_cids: Dict[str, str], session: aiohttp.ClientSession) -> bool:
    """Verify that the uploaded content is compatible with Stargaze."""
    logger.info("\n🔍 Verifying Stargaze compatibility...")
    
    # 1. Verify image CIDs are accessible
    logger.info("\nVerifying image CIDs...")
    for tile_id, cid in image_cids.items():
        try:
            url = f"{pinata.gateway_url}/{cid}"
            async with session.get(url) as response:
                if response.status != 200:
                    logger.error(f"❌ Image CID verification failed for tile {tile_id}: {cid}")
                    return False
                logger.info(f"✅ Image CID verified for tile {tile_id}: {cid}")
        except Exception as e:
            logger.error(f"❌ Failed to verify image CID for tile {tile_id}: {str(e)}")
            return False
    
    # 2. Verify metadata CIDs and structure
    logger.info("\nVerifying metadata CIDs and structure...")
    for tile_id, cid in metadata_cids.items():
        try:
            url = f"{pinata.gateway_url}/{cid}"
            async with session.get(url) as response:
                if response.status != 200:
                    logger.error(f"❌ Metadata CID verification failed for tile {tile_id}: {cid}")
                    return False
                
                metadata = await response.json()
                
                # Verify Stargaze metadata structure
                required_fields = ["name", "description", "image", "attributes"]
                missing_fields = [field for field in required_fields if field not in metadata]
                if missing_fields:
                    logger.error(f"❌ Metadata for tile {tile_id} missing required fields: {missing_fields}")
                    return False
                
                # Verify image CID in metadata matches our records
                image_cid = metadata["image"].replace("ipfs://", "")
                if image_cid != image_cids[tile_id]:
                    logger.error(f"❌ Image CID mismatch in metadata for tile {tile_id}")
                    logger.error(f"Expected: {image_cids[tile_id]}")
                    logger.error(f"Found: {image_cid}")
                    return False
                
                logger.info(f"✅ Metadata CID and structure verified for tile {tile_id}: {cid}")
        except Exception as e:
            logger.error(f"❌ Failed to verify metadata CID for tile {tile_id}: {str(e)}")
            return False
    
    # 3. Print final verification and instructions
    logger.info("\n✅ All CIDs verified successfully!")
    logger.info("\n📝 Stargaze Configuration:")
    logger.info(f'Collection URI: ipfs://QmXzzVdLPNZCG1RnCSxDCSu9TfFd7fpnnwt8GuXdjNkjZw')
    logger.info(f'Base Token URI: ipfs://{metadata_cids["1"]}/')
    logger.info("\nTo verify on Stargaze:")
    logger.info(f"1. Collection Image: https://ipfs.io/ipfs/QmXzzVdLPNZCG1RnCSxDCSu9TfFd7fpnnwt8GuXdjNkjZw")
    logger.info(f"2. Sample Metadata: https://ipfs.io/ipfs/{metadata_cids['1']}")
    logger.info(f"3. Sample Image: https://ipfs.io/ipfs/{image_cids['1']}")
    
    return True

async def upload_initial_content_async(pinata: PinataAPI) -> Tuple[str, Dict[str, str]]:
    """Upload initial content asynchronously and return CIDs."""
    # Prepare directory structure
    logger.info("🗂️ Preparing directory structure...")
    if not prepare_directory_structure():
        return None
    logger.info("✅ Directory structure prepared!")
    
    try:
        # First upload all images
        logger.info("\n📤 Uploading images...")
        image_cids = {}
        async with aiohttp.ClientSession() as session:
            image_tasks = []
            for image_file in sorted(Path("ipfs/images").glob("*.png"), key=lambda x: int(x.stem)):
                task = pinata.pin_file_async(session, str(image_file), f"image_{image_file.stem}")
                image_tasks.append(task)
            
            image_results = await asyncio.gather(*image_tasks)
            for token_id, cid in image_results:
                if cid:
                    image_cids[token_id] = cid
                    logger.info(f"✅ Uploaded image {token_id}: {cid}")
                else:
                    logger.error(f"❌ Failed to upload image {token_id}")
                    return None

        # Update metadata files with correct image CIDs
        logger.info("\n📝 Updating metadata with image CIDs...")
        metadata_cids = {}
        async with aiohttp.ClientSession() as session:
            metadata_tasks = []
            for metadata_file in sorted(Path("ipfs/metadata").glob("*.json"), key=lambda x: int(x.stem)):
                with open(metadata_file) as f:
                    metadata = json.load(f)
                    # Update image URI in metadata
                    token_id = metadata_file.stem
                    if token_id in image_cids:
                        metadata["image"] = f"ipfs://{image_cids[token_id]}"
                        task = pinata.pin_json_async(session, metadata, f"metadata_{token_id}")
                        metadata_tasks.append((token_id, task))
            
            for token_id, task in metadata_tasks:
                cid = await task
                if cid:
                    metadata_cids[token_id] = cid
                    logger.info(f"✅ Uploaded metadata {token_id}: {cid}")
                else:
                    logger.error(f"❌ Failed to upload metadata {token_id}")
                    return None

        # Verify uploads and compatibility
        async with aiohttp.ClientSession() as session:
            if not await verify_stargaze_compatibility(pinata, image_cids, metadata_cids, session):
                logger.error("❌ Verification failed")
                return None

        # Get first metadata CID for base URI
        first_metadata_cid = metadata_cids["1"]
        base_token_uri = f"ipfs://{first_metadata_cid}/"
        
        logger.info("\n📝 Stargaze Configuration:")
        logger.info(f"Base Token URI: {base_token_uri}")
        logger.info("\nTo verify on Stargaze:")
        logger.info(f"1. Sample Metadata: https://ipfs.io/ipfs/{first_metadata_cid}")
        logger.info(f"2. Sample Image: https://ipfs.io/ipfs/{image_cids['1']}")
        
        return first_metadata_cid, {
            "base_uri": base_token_uri,
            "metadata_cids": metadata_cids,
            "image_cids": image_cids
        }
        
    except Exception as e:
        logger.error(f"❌ Upload failed: {str(e)}")
        return None

async def main_async():
    # Check for environment variables
    api_key = os.getenv("PINATA_API_KEY")
    api_secret = os.getenv("PINATA_API_SECRET")
    
    if not api_key or not api_secret:
        print("❌ Error: Please set PINATA_API_KEY and PINATA_API_SECRET environment variables")
        print("\nTo set up:")
        print("1. Create an account at https://app.pinata.cloud/")
        print("2. Generate API keys at https://app.pinata.cloud/developers/api-keys")
        print("3. Add to your .env file:")
        print("   PINATA_API_KEY=your_api_key")
        print("   PINATA_API_SECRET=your_api_secret")
        return
    
    pinata = PinataAPI(api_key, api_secret)
    
    # Upload content
    result = await upload_initial_content_async(pinata)
    if result:
        content_cid, metadata_cids = result
        print("\n🔍 Verifying uploads...")
        
        print("\nFor metadata updates, use:")
        print("pinata.replace_pin(metadata_cid, new_metadata)")
        print("\nFor fetching metadata, use:")
        print("metadata = pinata.get_pin_json(metadata_cid)")
        print("\nFor bulk fetching, use:")
        print("metadata_dict = await pinata.get_all_metadata_async(token_ids)")

def main():
    asyncio.run(main_async())

if __name__ == "__main__":
    main() 