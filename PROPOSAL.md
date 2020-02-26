# rx-rust
## Team Members
- Chenxi Yuan, [yuanchenxi95](https://github.com/yuanchenxi95)
- Panyang Qi, [qipanyang](https://github.com/qipanyang)
- Bingyu Jiang, [jiangbryn](https://github.com/jiangbryn)
- Mingyue Shang, [shangmy](https://github.com/shangmy)

## Introduction
We propose to build an In-App messaging platform using rust. It is like the system firebase provides: https://firebase.google.com/products/in-app-messaging. We would like to build a micro service system that provides third party servers the ability let users send messages to each other. 

## Features
1. Peer-to-peer messages: Users send message to each other. Admins send message to any individual user. 
2. Peer-to-group messages: Admin send messages to a group of users who satisfy the particular filter statement.

## API Design

### User
1. Create User
2. Assign Admin
3. Block User
4. User preferences: Blacklist and whitelist

### Message
1. Send messages from user to user
2. Send messages from admin to users with a filter statement
3. Pull messages of a specified user
4. Mark messages as read

### Authentication
1. API Key generation and manage


## Nice to have
1. Support multi media messages, like form and interactive messages.
2. Retract messages
3. Channels support
