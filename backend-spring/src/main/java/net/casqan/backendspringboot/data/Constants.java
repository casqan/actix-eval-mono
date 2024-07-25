package net.casqan.backendspringboot.data;



public final class Constants {
    public static final String API_VERSION = "v1";
    public static final String API_BASE_URL = "/api/" + API_VERSION;
    public static final String PROFILE_BASE_URL = API_BASE_URL + "/profile";
    public static final String CHANNEL_BASE_URL = API_BASE_URL + "/channel";
    public static final String MESSAGE_BASE_URL = CHANNEL_BASE_URL + "{channelId}/message";
}
