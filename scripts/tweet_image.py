import argparse
import os
import tempfile

import cv2
import tweepy


def tweet_image(consumer_key: str,
                consumer_secret: str,
                access_token: str,
                access_token_secret: str,
                image_path: str,
                status: str) -> None:
    auth = tweepy.OAuthHandler(
        consumer_key=consumer_key, consumer_secret=consumer_secret)
    auth.set_access_token(access_token, access_token_secret)

    api = tweepy.API(auth)

    with tempfile.TemporaryDirectory() as tmp_dir:
        image_path = os.path.join(tmp_dir, 'render.jpg')
        im = cv2.imread('foo.ppm')
        cv2.imwrite(image_path, im)
        media = api.media_upload(image_path)
        api.update_status(status,
                          media_ids=[media.media_id])


if __name__ == '__main__':
    parser = argparse.ArgumentParser()

    parser.add_argument('consumer_key')
    parser.add_argument('consumer_secret')
    parser.add_argument('access_token')
    parser.add_argument('access_token_secret')
    parser.add_argument('image_path')
    parser.add_argument('status')

    args = parser.parse_args()

    tweet_image(args.consumer_key, args.consumer_secret,
                args.access_token, args.access_token_secret, args.image_path, args.status)
