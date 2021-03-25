pub fn fake_response() -> String {
    let response = r#"
    {
      "dog": "hi",
      "associated": {
        "units": {
          "temperature": "F",
          "tideHeight": "FT",
          "swellHeight": "FT",
          "waveHeight": "FT",
          "windSpeed": "KTS",
          "model": "LOLA"
        },
        "utcOffset": -10,
        "location": {
          "lon": -157.84106969833374,
          "lat": 21.280537020769913
        },
        "forecastLocation": {
          "lon": -157.8481,
          "lat": 21.282
        },
        "offshoreLocation": {
          "lon": -157.75,
          "lat": 21.25
        }
      },
      "data": {
        "wave": [
          {
            "timestamp": 1616407200,
            "utcOffset": -10,
            "surf": {
              "min": 1.84,
              "max": 2.76,
              "optimalScore": 0
            },
            "swells": [
              {
                "height": 0.59,
                "period": 11,
                "direction": 185.63,
                "directionMin": 181.41,
                "optimalScore": 0
              },
              {
                "height": 4.49,
                "period": 8,
                "direction": 85.78,
                "directionMin": 82.27,
                "optimalScore": 0
              },
              {
                "height": 0,
                "period": 0,
                "direction": 0,
                "directionMin": 0,
                "optimalScore": 0
              },
              {
                "height": 1.31,
                "period": 15,
                "direction": 168.75,
                "directionMin": 164.53,
                "optimalScore": 0
              },
              {
                "height": 0.59,
                "period": 5,
                "direction": 147.66,
                "directionMin": 144.84,
                "optimalScore": 0
              },
              {
                "height": 0,
                "period": 0,
                "direction": 0,
                "directionMin": 0,
                "optimalScore": 0
              }
            ]
          }
        ]
      }
    }"#
    .to_string();
    return response;
}
