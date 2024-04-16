from abc import ABC, abstractmethod
from typing import Any, Dict, Optional

from .dt_core_base_py import (
    init as dt_init,
    add_event as dt_add_event,
    flush as dt_flush,
    close as dt_close,
    toggle_logger as dt_toggle_logger
)

__SDK_NAME__ = "dt_server_sdk_python"


class Consumer(ABC):
    @abstractmethod
    def _get_config(self) -> dict:
        return NotImplemented


class DTAnalytics:
    def __init__(self, consumer: Consumer, debug=False):
        """ Initialize the DTAnalytics with given consumer.

        :param consumer: DTConsumer. e.g. DTLogConsumer.
        :param debug: If set to true, the data will not be inserted to production environment.
        """
        config = consumer._get_config()
        config["_debug"] = debug
        dt_init(config)

    def __add(self, dt_id: str, acid: Optional[str], event_name: str, event_type: str,
              properties: Dict[str, Any]) -> bool:
        event = dict(properties)
        event["#dt_id"] = dt_id
        if acid is not None:
            event["#acid"] = acid
        event["#event_name"] = event_name
        event["#event_type"] = event_type
        event["#sdk_type"] = __SDK_NAME__

        return dt_add_event(event)

    def track(self, dt_id: str, acid: Optional[str], event_name: str, properties: Dict[str, Any]) -> bool:
        """ Track an event.

        :param dt_id: The device-scoped id.
        :param acid: The account-scoped id.
        :param event_name: Event name, can be custom event or preset event.
        :param properties: properties of this event. (preset properties are scoped by event name, and has type constraints)
        :return: True if given event and properties is valid or False if there are invalid and will not be processed.
        """
        return self.__add(dt_id, acid, event_name, "track", properties)

    def user_set(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        """ Set user properties for the user with given dtId and acId.

        :param dt_id: The device-scoped id.
        :param acid: The account-scoped id.
        :param properties: properties of this event. (preset properties are scoped by event name, and has type constraints)
        :return: True if given event and properties is valid or False if there are invalid and will not be processed.
        """
        return self.__add(dt_id, acid, "#user_set", "user", properties)

    def user_set_once(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        """ Set user properties for the user with given dtId and acId.
        The value will not override existed property.

        :param dt_id: The device-scoped id.
        :param acid: The account-scoped id.
        :param properties: properties of this event. (preset properties are scoped by event name, and has type constraints)
        :return: True if given event and properties is valid or False if there are invalid and will not be processed.
        """
        return self.__add(dt_id, acid, "#user_set_once", "user", properties)

    def user_add(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        """ Arithmetic add the value of property by given number for user with given dtId and acId.
        Hence, the type of value for 'custom properties' should be a number.

        :param dt_id: The device-scoped id.
        :param acid: The account-scoped id.
        :param properties: properties of this event. (preset properties are scoped by event name, and has type constraints)
        :return: True if given event and properties is valid or False if there are invalid and will not be processed.
        """
        return self.__add(dt_id, acid, "#user_add", "user", properties)

    def user_unset(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        """ Unset properties for user with given dtId and acId.
        Only the key of 'custom properties' will be used and its value is meaningless here.

        :param dt_id: The device-scoped id.
        :param acid: The account-scoped id.
        :param properties: properties of this event. (preset properties are scoped by event name, and has type constraints)
        :return: True if given event and properties is valid or False if there are invalid and will not be processed.
        """
        props_dict = {}
        for k, v in properties.items():
            if k.startswith("#"):
                props_dict[k] = v
            else:
                props_dict[k] = 0
        return self.__add(dt_id, acid, "#user_unset", "user", props_dict)

    def user_delete(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        """ Delete the user with given dtId and acId.

        :param dt_id: The device-scoped id.
        :param acid: The account-scoped id.
        :param properties: preset properties of this event.
        :return: True if given event and properties is valid or False if there are invalid and will not be processed.
        """
        return self.__add(dt_id, acid, "#user_delete", "user", properties)

    def user_append(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        """ Append values to property for the user with given dtId and acId.
        Hence, the type of value for 'custom properties' should be an array.

        :param dt_id: The device-scoped id.
        :param acid: The account-scoped id.
        :param properties: properties of this event. (preset properties are scoped by event name, and has type constraints)
        :return: True if given event and properties is valid or False if there are invalid and will not be processed.
        """
        return self.__add(dt_id, acid, "#user_append", "user", properties)

    def user_uniq_append(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        """ Append values to property without duplications for the user with given dtId and acId.
        Hence, the type of value for 'custom properties' should be an array.

        :param dt_id: The device-scoped id.
        :param acid: The account-scoped id.
        :param properties: properties of this event. (preset properties are scoped by event name, and has type constraints)
        :return: True if given event and properties is valid or False if there are invalid and will not be processed.
        """
        return self.__add(dt_id, acid, "#user_uniq_append", "user", properties)

    def toggle_logger(self, enable: bool):
        """ To enable and disable the logging. """
        dt_toggle_logger(enable)

    @staticmethod
    def enable_log():
        """ To enable the logging. """
        dt_toggle_logger(True)

    @staticmethod
    def disable_log():
        """ To disable the logging. """
        dt_toggle_logger(False)

    def flush(self):
        """ Flush the data buffer manually. """
        dt_flush()

    def close(self):
        """ Close the DTAnalytics, remember to call this before the program finishes to preventing data loss! """
        dt_close()


class DTLogConsumer(Consumer):
    def __init__(self, path, max_batch_len, name_prefix, max_file_size_bytes):
        """ Creates an DTConsumer to log the events.
        Event logs will be stored in the path with name_prefix.
        The inputted event will be buffered and be written to log per every max_batch_len length.
        Also, the event log is sharding by either of these circumstances:
            - Every hour,
            - File size is over approximated maximum file size max_file_size_bytes in bytes (0 for unlimited).

        :param path: The path/directory to store event logs, will be created if not exist.
        :param max_batch_len: maximum number of events to be written to log once.
        :param name_prefix: prefix of log file.
        :param max_file_size_bytes: approximated maximum file size in bytes. (will be exceeded if the actual event size is larger)
        """
        self.__config = {
            "consumer": "log",
            "path": path,
            "max_batch_len": max_batch_len,
            "name_prefix": name_prefix,
            "max_file_size_bytes": max_file_size_bytes,
        }

    def _get_config(self):
        return self.__config
