import requests

HEADERS = {
    "User-Agent": "Mozilla/5.0",
    "Referer": "https://leetcode.com/",
    # "Cookie": f"LEETCODE_SESSION={LEETCODE_SESSION}"
}

URL = "https://leetcode.com/graphql"


def fetch_daily_info():
    daily_query = {
        "query": """
        query questionOfToday {
            activeDailyCodingChallengeQuestion {
                question {
                    questionFrontendId
                    title
                    titleSlug
                }
            }
        }
        """
    }
    r = requests.post(URL, json=daily_query, headers=HEADERS)
    return r.json()["data"]["activeDailyCodingChallengeQuestion"]["question"]


def make_daily_dir(daily_info) -> Path:
    qid = daily_info["questionFrontendId"]
    slug = daily_info["titleSlug"]

    daily_dir = LEETCODE_DIR / f"{qid}-{slug}"
    daily_dir.mkdir(parents=True, exist_ok=True)
    return daily_dir


from pathlib import Path

LEETCODE_DIR = Path(__file__).resolve().parent / "leetcode"

if __name__ == "__main__":
    daily_info = fetch_daily_info()
    daily_dir = make_daily_dir(daily_info)
    print(daily_dir)
