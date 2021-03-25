# Surfbot

This is an app that uses the Surfline API to show surf conditions in the mac touchbar.


https://services.surfline.com/kbyg/spots/forecasts/wave?spotId=5842041f4e65fad6a7708b39

### Surfline API

This app makes requests to the new Surfline API, which is publicly available at the endpoint like this:

```
https://services.surfline.com/kbyg/spots/forecasts/{type}?{params}
```

Where `type` is one of `wave, wind, tides, weather`, and `params` is like the below:

Param|Values|Effect
-----|------|------
spotId|string|Surfline spot id that you want data for. A typical Surfline URL is `https://www.surfline.com/surf-report/venice-breakwater/590927576a2e4300134fbed8` where `590927576a2e4300134fbed8` is the `spotId`
days|integer|Number of forecast days to get (Max 6 w/o access token, Max 17 w/ premium token)
intervalHours|integer|Minimum of 1 (hour)
maxHeights|boolean|`true` seems to remove min & optimal values from the wave data output
sds|boolean|If true, use the new LOTUS forecast engine
accesstoken|string|Auth token to get premium data access (optional)

Calling the API for `wave` returns a nested response like this:
```json
{
  "data": {
    "wave": [
      {
        "timestamp":1616328000,
        "utcOffset":-10,
        "surf":{
           "min":1.21,
           "max":2.23,
           "optimalScore":0
        },
        "swells":[
          {
            "height":4.59,
            "period":8,
            "direction":84.38,
            "directionMin":80.86,
            "optimalScore":0
          },
          {
            "height":0.49,
            "period":4,
            "direction":143.44,
            "directionMin":139.92,
            "optimalScore":0
          }
        ]
      }
    ]
  }
}
```

### To-do's

- [ ] package for touchbar native api's