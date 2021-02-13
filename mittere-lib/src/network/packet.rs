pub enum PacketType {
    // base packet layout: <packet id ##>|<data>
    // where packet id ## is the packet identifier and data is the data to send / receive
    // note to self: dont need a separator between packet id and data, packet id is always 2 chars
    // note: in the layout, | is the PACKET_VAR_SEPARATOR

    // client -> server
    // sends a login request to the server
    // variables are self-explanatory...
    // layout: 00|<username>|<password>
    LOGIN,

    // server -> client
    // sends if the login was accepted by the server
    // data is either 0 or 1, with 0 being denied and 1 being accepted
    // layout: 01|<data>
    LOGIN_VALIDATE,

    // client -> server
    // sends to the server to check if key is valid
    // key is the key being used by the client (see ClientKey)
    // layout: 02|<key>
    SIGNUP_KEY,

    // server -> client
    // tells the client if the key was accepted
    // data is either 0 or 1, with 0 being denied and 1 being accepted
    // layout: 03|<data>
    SIGNUP_KEY_VALIDATE,

    // client -> server
    // tells the server the signup data
    // again, variables are self-explanatory
    // layout: 04|<username><password>
    SIGNUP,

    // server -> client
    // tells the client if the key was accepted
    // data is either 0 or 1, with 0 being denied and 1 being accepted
    // layout: 05|<data>
    SIGNUP_VALIDATE,

    // client -> server
    // tells the server if the client switches channels
    // channel is the channel to switch to
    // layout: 06|<channel>
    SWITCH_CHANNEL,

    // client -> server
    // tells the server if the client switches channels
    // data is either 0 or 1, with 0 being denied and 1 being accepted
    // layout: 07|<data>
    SWITCH_CHANNEL_VALIDATE,

    // both ways
    // tells the server that the client has requested to disconnect (not forced)
    // tells the client either the disconnect has been received or is being forced
    // no variables, this is more of a flag
    // layout: 08
    DISCONNECT,

    // server -> all
    // tells the clients that either another client or the server has a message to be displayed
    //   in the current channel
    // variables should explain themselves, yet again (totally not lazy)
    // layout: 09|<display name>|<name color>|<msg>|<msg color>
    OTHER_SENT_MSG,

    // client -> server
    // tells the server the user sent a message
    // msg is the message sent
    // layout: 10|<msg>
    SEND_MSG,

    // server -> client
    // requests config information used by the server from the client
    // layout: 11
    REQUEST_CONFIG_UPDATE,

    // client -> server
    // sends a config update to the server
    // will update all config values on the server for the user even if not requested
    // layout: 12|<display name>|<name color>|<msg color>
    CONFIG_UPDATE,

    // server -> client
    // sends a packet telling the client that the server is full
    // layout: 13
    SERVER_FULL,

    // server -> client
    // sends the Message Of The Day on connection
    // layout: 14|<motd>
    MOTD,

    // client -> server
    // tells the server that a client has attempted to run an admin command on the server
    // layout: 69|<cmd>
    ADMIN_CMD,

    // both ways
    // tells the server that the client has encountered an error and to safely disconnect
    // tells the client that the server has encountered an error with the connection and should disconnect
    // also used when a packet is not identifiable
    // layout: 15 or any non-valid packet id
    ERROR
}

impl PacketType {
    pub fn get_type_id(&self) -> u8 {
        match self {
            PacketType::LOGIN                   => 00,
            PacketType::LOGIN_VALIDATE          => 01,
            PacketType::SIGNUP_KEY              => 02,
            PacketType::SIGNUP_KEY_VALIDATE     => 03,
            PacketType::SIGNUP                  => 04,
            PacketType::SIGNUP_VALIDATE         => 05,
            PacketType::SWITCH_CHANNEL          => 06,
            PacketType::SWITCH_CHANNEL_VALIDATE => 07,
            PacketType::DISCONNECT              => 08,
            PacketType::OTHER_SENT_MSG          => 09,
            PacketType::SEND_MSG                => 10,
            PacketType::REQUEST_CONFIG_UPDATE   => 11,
            PacketType::CONFIG_UPDATE           => 12,
            PacketType::SERVER_FULL             => 13,
            PacketType::MOTD                    => 14,
            PacketType::ERROR                   => 15, // although unknown can be any number that is not a valid packet, it is set to 15 here just to satisfy the compiler :)
            PacketType::ADMIN_CMD               => 69
        }
    }
    pub fn from_type_id(id: u8) -> PacketType {
        match id {
            00 => PacketType::LOGIN,
            01 => PacketType::LOGIN_VALIDATE,
            02 => PacketType::SIGNUP_KEY,
            03 => PacketType::SIGNUP_KEY_VALIDATE,
            04 => PacketType::SIGNUP,
            05 => PacketType::SIGNUP_VALIDATE,
            06 => PacketType::SWITCH_CHANNEL,
            07 => PacketType::SWITCH_CHANNEL_VALIDATE,
            08 => PacketType::DISCONNECT,
            09 => PacketType::OTHER_SENT_MSG,
            10 => PacketType::SEND_MSG,
            11 => PacketType::REQUEST_CONFIG_UPDATE,
            12 => PacketType::CONFIG_UPDATE,
            13 => PacketType::SERVER_FULL,
            14 => PacketType::MOTD,
            69 => PacketType::ADMIN_CMD,
            _ => PacketType::ERROR,
        }
    }
}