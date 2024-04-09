from abc import ABC, abstractmethod
from typing import Any, Dict, Optional
import sys

from .dt_core_base_py import (
    init as dt_init,
    add_event as dt_add_event,
    flush as dt_flush,
    close as dt_close,
    toggle_logger as dt_toggle_logger
)

version = sys.version_info
if version >= (3, 8):
    import importlib.metadata

    __VERSION__ = importlib.metadata.version("dt_core_python")
else:
    try:
        from importlib_metadata import version as metadata_version

        __VERSION__ = metadata_version("dt_core_python")
    except ModuleNotFoundError as e:
        print(f"ModuleNotFoundError: {e.msg}")
        print("Please run `pip install importlib_metadata` to install such prerequisite!")
        sys.exit(1)

__SDK_NAME__ = "dt_python_sdk"


class Consumer(ABC):
    @abstractmethod
    def _get_config(self) -> dict:
        return NotImplemented


class DTAnalytics:
    def __init__(self, consumer: Consumer, debug=False):
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
        event["#sdk_version_name"] = __VERSION__

        return dt_add_event(event)

    def track(self, dt_id: str, acid: Optional[str], event_name: str, properties: Dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, event_name, "track", properties)

    def user_set(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_set", "user", properties)

    def user_set_once(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_set_once", "user", properties)

    def user_add(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_add", "user", properties)

    def user_unset(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        props_dict = {}
        for k, v in properties.items():
            if k.startswith("#"):
                props_dict[k] = v
            else:
                props_dict[k] = 0
        return self.__add(dt_id, acid, "#user_unset", "user", props_dict)

    def user_delete(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_delete", "user", properties)

    def user_append(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_append", "user", properties)

    def user_uniq_append(self, dt_id: str, acid: Optional[str], properties: Dict[str, Any]) -> bool:
        return self.__add(dt_id, acid, "#user_uniq_append", "user", properties)

    def toggle_logger(self, enable: bool):
        dt_toggle_logger(enable)

    @staticmethod
    def enable_log():
        dt_toggle_logger(True)

    @staticmethod
    def disable_log():
        dt_toggle_logger(False)

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
