Personal Web Server Made in RUST 

Built using information learned from "RUST Programming Lanugage Book" and using RUST docs.

Have a website in which people can register an account with the server, or is automatically prompted with a password entering to sign-in. 

New Users can either be SUBSCRIBERS, or PUBLISHERS

Client-side:
Publishers: Can publish new data to the server for a given topic.
Subscribers: Can subscribe to different news topics to be updated about, receives new publishings over time. Can also unsubscribe from topics to not see them anymore

Server-side: 
Keeps track of user data including names, passwords, subscribers vs publishers, Subscribed/Publisher topics. Will also collect data about the number of people subscribed to topics and number of messages seen for each user to demonstrate how server collected data (cookies) can be used. 

Planned Additions:
1. Build a storage for messages and subscribed users, set up commands to subscribe and publish (or via a website)
2. Allow messages to be received by a subscriber at any time using polling
3. Allow the server to only forward published messages when users are online, banks messages when users are offline
4. Track "cookie" data.
