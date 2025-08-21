import requests
from faker import Faker # pyright: ignore[reportMissingImports]
import random
import argparse
import sys

fake = Faker()

def safe_request(session: requests.Session, method: str, url: str, *, json=None, timeout=10):
    try:
        resp = session.request(method, url, json=json, timeout=timeout)
        return resp
    except requests.exceptions.RequestException as e:
        print(f"❌ {method} {url} -> Request error: {e}")
        return None






def login(session: requests.Session, base_url: str, username: str, password: str, timeout=10):
    login_url = f"{base_url}/api/login"
    payload = {"username": username, "password": password}

    resp = safe_request(session, "POST", login_url, json=payload, timeout=timeout)
    if not resp:
        return False

    if resp.status_code not in (200, 204):
        print(f"❌ Login failed ({resp.status_code}): {resp.text}")
        return False

    # Debug cookies
    if session.cookies:
        print(f"✅ 🚪 User has logged in")
        # for c in session.cookies:
        #     print(f"✅ Cookie stored: {c.name}={c.value} (secure={c.secure}, domain={c.domain})")
    else:
        print("⚠️ Logged in but no cookies were set (maybe Secure over HTTP?)")

    return True






def create_products(
        session: requests.Session, 
        base_url: str, 
        n: int, 
        timeout=10
    ):
    product_url = f"{base_url}/api/admin/products/new"

    for i in range(1, n + 1):
        product = {
            "name": fake.word().capitalize(),
            "description": fake.sentence(),
            "price": round(random.uniform(10, 500), 2),  # decimal(2)
            "stock": random.randint(1, 1000),            # i32
        }

        resp = safe_request(session, "POST", product_url, json=product, timeout=timeout)

        if resp.status_code in (200, 201):
            print(f"[{i}/{n}] ✅ Created: {product['name']}")
        else:
            print(f"[{i}/{n}] ❌ {resp.status_code}: {resp.text}")



def create_bundles(session: requests.Session, base_url: str, n: int):
    """
    Create a new bundle by randomly selecting products from the product list.

    Args:
        session (requests.Session): The session object with authentication/cookies set up.
        base_url (str): The base API URL (e.g., "http://localhost:8000").
        bundle_name (str): Name of the bundle.
        bundle_price (float): Price of the bundle.
        num_items (int): Number of items to include in the bundle.
    """
    # Step 1: Get product list
    products_url = f"{base_url}/api/admin/products/list?limit=20&page=1"
    resp = session.get(products_url)

    if resp.status_code != 200:
        print("Failed to fetch products:", resp.text)
        return

    data = resp.json()

    # Adjust depending on your API response
    products = data.get("success", data).get("list", data)  # if "items" exists, use that, else assume it's already a list

    if not isinstance(products, list) or not products:
        print("No products available to create a bundle.")
        return


    for i in range(1, n+1):
        # Step 2: Randomly select products
        selected_products = random.sample(products, min(random.randint(1, 5), len(products)))

        # Step 3: Build items array (assign random quantities 1-3 for demo)
        items = [
            {"product_id": p["id"], "quantity": random.randint(1, 20)}
            for p in selected_products
        ]

        # Step 4: Create bundle payload
        payload = {
            "name": fake.word(),
            "price": round(random.uniform(10, 500), 2),
            "items": items
        }

        # Step 5: Send request to create bundle
        bundle_url = f"{base_url}/api/admin/bundles/new"
        response = safe_request(session, "POST", bundle_url, json=payload)

        if response.status_code == 201:
            print(f"[{i}/{n}] ✅ Bundle created successfully:")
        else:
            print(f"[{i}/{n}] ✅Failed to create bundle:", response.status_code, response.text)




def get_arguments() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Create products for testing")
    p.add_argument("--base-url", default="https://localhost:9494", help="Service base URL")
    p.add_argument("--product-amount", type=int, default=0, help="Number of products to create")
    p.add_argument("--bundle-amount", type=int, default=0, help="Number of products to create")
    p.add_argument("--username", required=True, help="Login username")
    p.add_argument("--password", required=True, help="Login password")
    p.add_argument("--timeout", type=int, default=10, help="Per-request timeout (seconds)")
    return p.parse_args()






if __name__ == "__main__":
    args = get_arguments()

    # One session = persistent cookies + connection reuse
    with requests.Session() as s:
        # Optional: identify the client in logs
        s.headers.update({"User-Agent": "dev-seeder/1.0"})
        s.verify = "../cert.pem"

        if not login(s, args.base_url, args.username, args.password, timeout=args.timeout):
            print("❌ Cannot proceed without login (wrong credentials or server not reachable).")
            sys.exit(1)

        # Seeds products to the service
        create_products(s, args.base_url, args.product_amount, timeout=args.timeout)

        # Seeds Bundles to the service
        create_bundles(s, args.base_url, args.bundle_amount)
