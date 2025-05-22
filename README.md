# Reflection

## 3.1. Original code

This is a test run of the Yew WebChat client based on the tutorial.

- Step 1 :

I open a new web window, login as 'Azim', and click “Go Chatting!”.
I join the chat alone:

![3.1 - 1](images/3.1_1.png)
![3.1 - 2](images/3.1_2.png)


- Step 2 :

I open a new web window, login as 'Adi', and click “Go Chatting!”.
I join the chat with Azim but I can't see the previous message from Azim because I was not in the chat session yet.
Azim session sees the new user:

Adi's window:
![3.1 - 3](images/3.1_3.png)
![3.1 - 4](images/3.1_4.png)

Azim's window:
![3.1 - 5](images/3.1_5.png)

## 3.2. Add some creativities to the webclient

### 1st feature : Add User Icon

I added the ability for users to choose a custom avatar when they join the chat. Instead of using generic randomly generated icons, users can now select from a list of bird icons during login:
![3.2 - 1](images/3.2_1.png)

Once connected, each user's avatar is shown:
- In the user list (left sidebar):
![3.2 - 2](images/3.2_2.png)

- Next to each of their messages in the chat window:
![3.2 - 3](images/3.2_3.png)


To do this:

- I modified the login page to let users choose an avatar (from 6 bird icons).

- I updated the WebSocket messages so that each user sends not only their name, but also their chosen avatar.

- The server then broadcasts both name and avatar to all clients.

- The client UI was updated to display avatars in the chat and user list.