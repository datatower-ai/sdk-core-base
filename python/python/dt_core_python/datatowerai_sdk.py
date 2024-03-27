from abc import ABC, abstractmethod
from typing import Any, Final, Optional
import importlib.metadata

from .dt_core_base_py import init as dt_init, add_event as dt_add_event, flush as dt_flush, close as dt_close, toggle_logger

__SDK_NAME__: Final[str] = "dt_python_sdk"
__VERSION__ = importlib.metadata.version("dt_core_python")


class Consumer(ABC):
    @abstractmethod
    def _get_config(self) -> dict:
        return NotImplemented


class DTAnalytics:
    def __init__(self, consumer: Consumer, debug=False):
        toggle_logger(debug)
        dt_init(consumer._get_config())

    def __add(self, dt_id: str, acid: Optional[str], event_name: str, event_type: str, properties: dict[str, Any]) -> bool:
        event = dict(properties)
        event["#dt_id"] = dt_id
        if acid is not None:
            event["#acid"] = acid
        event["#event_name"] = event_name
        event["#event_type"] = event_type
        event["#sdk_type"] = __SDK_NAME__
        event["#sdk_version_name"] = __VERSION__
        return dt_add_event(event)

    def track(self, dt_id: str, acid: Optional[str], event_name: str, properties: dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, event_name, "track", properties)

    def user_set(self, dt_id: str, acid: Optional[str], properties: dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_set", "user", properties)

    def user_set_once(self, dt_id: str, acid: Optional[str], properties: dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_set_once", "user", properties)

    def user_add(self, dt_id: str, acid: Optional[str], properties: dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_add", "user", properties)

    def user_unset(self, dt_id: str, acid: Optional[str], properties: list[str]) -> bool:
        props_dict = {}
        for prop in properties:
            props_dict[prop] = 0
        return self.__add(dt_id, acid, "#user_unset", "user", props_dict)

    def user_delete(self, dt_id: str, acid: Optional[str]) -> bool:
        return self.__add(dt_id, acid, "#user_delete", "user", {})

    def user_append(self, dt_id: str, acid: Optional[str], properties: dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_append", "user", properties)

    def user_uniq_append(self, dt_id: str, acid: Optional[str], properties: dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_uniq_append", "user", properties)

    def flush(self):
        dt_flush()

    def close(self):
        dt_close()


class DTLogConsumer(Consumer):
    def __init__(self, path, max_batch_len, name_prefix, max_file_size_bytes):
        self.__config = {
            "consumer": "log",
            "path": path,
            "max_batch_len": max_batch_len,
            "name_prefix": name_prefix,
            "max_file_size_bytes": max_file_size_bytes,
        }

    def _get_config(self):
        return self.__config
