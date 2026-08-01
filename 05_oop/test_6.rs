    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};

    use chrono::{DateTime, Utc};

    pub type SharedUser = Rc<RefCell<User>>;
    pub type SharedRoom = Rc<RefCell<Room>>;
    pub type SharedMessage = Rc<RefCell<Message>>;

    pub struct ChatApp {
        name: String,
        users: Vec<SharedUser>,
        rooms: Vec<SharedRoom>,
    }

    impl ChatApp {
        pub fn new(name: String) -> ChatApp {
            ChatApp {
                name,
                users: Vec::new(),
                rooms: Vec::new(),
            }
        }

        pub fn add_user(&mut self, user: SharedUser) {
            let user_id = user.borrow().id();
            let exists = self.users.iter().any(|u| u.borrow().id() == user_id);

            if !exists {
                self.users.push(user);
            }
        }

        pub fn add_room(&mut self, room: SharedRoom) {
            let room_name = room.borrow().name().to_string();
            let exists = self.rooms.iter().any(|r| r.borrow().name() == room_name);

            if !exists {
                self.rooms.push(room);
            }
        }

        pub fn remove_user(&mut self, user: &SharedUser) {
            for room in &self.rooms {
                room.borrow_mut().remove_user(user);
            }
            self.users.retain(|u| !Rc::ptr_eq(u, user));
            let mut user_ref = user.borrow_mut();
            user_ref.set_offline();
            user_ref.chats.clear();
        }

        pub fn all_users(&self) -> &[SharedUser] {
            &self.users
        }

        pub fn all_rooms(&self) -> &[SharedRoom] {
            &self.rooms
        }

        pub fn name(&self) -> &str {
            &self.name
        }
    }

    pub struct User {
        id: u32,
        name: String,
        online: bool,
        chats: Vec<String>,
    }

    impl User {
        pub fn new(name: String) -> User {
            static NEXT_ID: AtomicU32 = AtomicU32::new(1);
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

            User {
                id,
                name,
                online: false,
                chats: Vec::new(),
            }
        }

        fn set_online(&mut self) {
            self.online = true;
        }

        fn set_offline(&mut self) {
            self.online = false;
        }

        fn add_chat(&mut self, chat: &str) {
            if !self.chats.iter().any(|c| c == chat) {
                self.chats.push(chat.to_string());
            }
            self.set_online();
        }

        fn leave_chat(&mut self, chat: &str) {
            if let Some(pos) = self.chats.iter().position(|c| c == chat) {
                self.chats.remove(pos);
            }

            if self.chats.is_empty() {
                self.set_offline();
            }
        }

        pub fn id(&self) -> u32 {
            self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn is_online(&self) -> bool {
            self.online
        }

        pub fn chats(&self) -> &[String] {
            &self.chats
        }
    }

    pub struct Room {
        name: String,
        users: Vec<SharedUser>,
        messages: Vec<SharedMessage>,
    }

    impl Room {
        pub fn new(name: String) -> Room {
            Room {
                name,
                users: Vec::new(),
                messages: Vec::new(),
            }
        }

        pub fn add_user(&mut self, user: SharedUser) {
            let user_id = user.borrow().id();
            let exists = self.users.iter().any(|u| u.borrow().id() == user_id);

            if !exists {
                self.users.push(Rc::clone(&user));
                user.borrow_mut().add_chat(&self.name);
            }
        }

        pub fn remove_user(&mut self, user: &SharedUser) {
            self.users.retain(|u| !Rc::ptr_eq(u, user));
            user.borrow_mut().leave_chat(&self.name);
        }

        pub fn has_user(&self, user: &SharedUser) -> bool {
            let user_id = user.borrow().id();
            self.users.iter().any(|u| u.borrow().id() == user_id)
        }

        pub fn post_message(&mut self, user: SharedUser, text: String) {
            if self.has_user(&user) {
                let message = Rc::new(RefCell::new(Message::new(user, text)));
                self.messages.push(message);
            }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn users(&self) -> &[SharedUser] {
            &self.users
        }

        pub fn messages(&self) -> &[SharedMessage] {
            &self.messages
        }
    }

    pub struct Message {
        author: SharedUser,
        text: String,
        time: DateTime<Utc>,
    }

    impl Message {
        pub fn new(user: SharedUser, text: String) -> Message {
            Message {
                author: user,
                text,
                time: Utc::now(),
            }
        }

        pub fn text(&self) -> &str {
            &self.text
        }

        pub fn time(&self) -> DateTime<Utc> {
            self.time
        }

        pub fn author(&self) -> SharedUser {
            Rc::clone(&self.author)
        }
    }
