---
title: AWS lambda authorizer caching
meta_description: AWS lambda authorizers cache the policy document, so make sure that policy works for all endpoints it covers
---

# AWS lambda authorizer caching

## Problem

1. Make a request to an endpoint in API gateway that uses a lambda authorizer
	 *with caching enabled*. This request succeeeds.
2. Make a request to a *different* endpoint in API gateway that uses the same
	 lambda authorizer. This request fails with an error that looks something like
   this:

	 ```json
	 {
       "message": "User is not authorized to access this resource"
	 }
	 ```

## Why?

When a lambda authorizer has caching enabled, it uses the authorization token
(i.e. `Bearer abc123`) as a key and a policy document as a value. The policy
document looks something like:

```json
{
  "principalId": "user",
  "policyDocument": {
    "Statement": [
      {
        "Action": "execute-api:Invoke",
        "Effect": "Allow",
        "Resource": [
          "arn:aws:execute-api:us-1:abc:123/prod/GET/v1/users",
        ]
      }
    ],
    "Version": "2012-10-17"
  }
}
```

So, if the `Resource` in the policy is specific to single endpoint, the
cached value will only work for that endpoint. I was using the `methodArn` to
dynamically generate this policy document on each invocation, which is why I
was having my problem - the cached policy would only work for the first endpoint
I hit. I'd have to wait until the cache was invalidated before I could hit a
different endpoint... and then wait longer to be able to hit another.

## Solutions

1. (*bad*) Disable authorizer caching. This is probably not what most people want
	 since calls to the authorizer will likely involve calling a separate identity
	 service.
2. Make the `Resource`s covered by the policy more lax. Ideally using wildcards
   in portions of the resources

	 ```json
	 "Resource": "arn:aws:execute-api:us-1:abc:123/prod/*/v?/*"	
	 ```
	 
	 or just allowing any resource to be invoked
	
	 ```json
	 "Resource": "*"
	 ```
		
	 (which isn't too awful since the `Action` portion of the policy is pretty
   restrictive).

### References

- [This SO post](https://stackoverflow.com/questions/50331588/aws-api-gateway-custom-authorizer-strange-showing-error)

