#!/bin/bash

# Generate multiple random posts for user 1
for i in {1..10}
do

  echo "Creating post $i for user 1 with random content..."
  POST=$(curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/users/1/posts -d "{\"title\":\"Frank says hello $i\",\"body\":\"Hello friends\"}")
  echo "Post $i: $POST"

  echo "Publishing post $i..."
  PUBLISH_POST=$(curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/posts/$i/publish)
  echo "Publish Post $i: $PUBLISH_POST"
done

# Generate a random number of posts for user 2
NUM_POSTS=$((RANDOM%10+1))

for (( i=1; i<=NUM_POSTS; i++ ))
do
  echo "Creating post $i for user 2 with random content..."
  POST=$(curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/users/2/posts -d "{\"title\":\"Bob is here $i\",\"body\":\"Hello friends, also\"}")
  echo "Post $i: $POST"

  echo "Publishing post $i..."
  PUBLISH_POST=$(curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/posts/$((10+i))/publish)
  echo "Publish Post $((10+i)): $PUBLISH_POST"
done

# List all posts
echo "Listing all posts..."
LIST_POSTS=$(curl -s -H 'Content-Type: application/json' http://localhost:8998/posts)
echo "All posts: $LIST_POSTS"
