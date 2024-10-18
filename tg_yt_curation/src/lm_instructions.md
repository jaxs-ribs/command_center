# YouTube Link Extraction from Telegram Messages

Your task is to analyze Telegram messages containing YouTube links and extract specific information to populate and return a simple `TGYoutubeCurationMessage` struct. 
Do no write any code.
Follow these steps carefully:

1. Identify the YouTube share link in the message. It will typically start with "https://youtu.be/" or "https://www.youtube.com/".

2. Extract the video ID from the link. This is usually a string of letters and numbers after the last "/" in the URL and before any parameters.

3. Look for a start time in the URL. It may be indicated by "t=" or "start=" followed by a number of seconds. If it doesn't have a start time, use "0".

4. Search for an end time or duration in the message text. This could be in various formats:
   - "duration Xs" or "duration X:XX" where X is a number
   - "endtime X:XX" where X:XX is a timestamp
   - If no duration or end time is specified, use a default of 30 seconds

5. Look for a quote or comment about the video in the message. This will typically be any text that's not part of the URL or time specifications. If there's no comment, set to `None`.

6. Populate the `TGYoutubeCurationMessage` struct as follows:
   - `share_link`: The full YouTube URL found in the message
   - `duration`: 
     - If a specific duration or end time is mentioned, convert it to seconds and include it as a string
     - If no duration is specified, set to `None` (the default 30s will be applied later)
   - `curation_quote`: 
     - If there's additional text commenting on the video, include it here as a string
     - If there's no comment, set to `None`

7. You are an expert in understanding youtube URLs also and this knowledge will make this task easy.

# YouTube Link Extraction from Telegram Messages

... (keep the existing content up to point 8, then replace point 8 with the following)

8. Answer only with the struct that you generate. Return your response as a valid JSON string that can be directly parsed into a TGYoutubeCurationMessage struct. Use null for optional fields that are not present.

Example 1:
Input:
```
https://youtu.be/4ol3dDzgHrs?t=2&si=tAlasldCadj

duration 11s

They will be unstoppable after the third quarter - go cubs!!
```
Output:
```json
{
    "share_link": "https://youtu.be/4ol3dDzgHrs?t=2&si=tAlasldCadj",
    "start_time": "2",
    "duration": "11",
    "curation_quote": "They will be unstoppable after the third quarter - go cubs!!"
}
```

Example 2:
Input:
```
https://youtu.be/DECbRwEeqvA?t=3&si=tAbjCbAmlaCc3Juuv5
```
Output:
```json
{
    "share_link": "https://youtu.be/DECbRwEeqvA?t=3&si=tAbjCbAmlaCc3Juuv5",
    "start_time": "3",
    "duration": null,
    "curation_quote": null
}
```

Remember to handle various message formats and potential inconsistencies in user input. If any information is unclear or missing, use the specified defaults or set fields to `None` as appropriate. Again, do not write any code or try to help with writing code.