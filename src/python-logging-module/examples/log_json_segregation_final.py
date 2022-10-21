
import sys
import logging
from pythonjsonlogger import jsonlogger
from requests import TooManyRedirects


class CustomJsonFormatter(jsonlogger.JsonFormatter):
    def add_fields(self, log_record, record, message_dict):
        super(CustomJsonFormatter, self).add_fields(
            log_record, record, message_dict)
        log_record["level"] = record.levelname
        log_record["name"] = record.name
        log_record["message"] = record.getMessage()
        if record.levelname == "ERROR" or record.levelname == "EXCEPTION":
            log_record["exception"] = record.exc_info
            log_record["stacktrace"] = record


ROOT_LEVEL = logging.DEBUG


class StdoutFilter(logging.Filter):
    def filter(self, record):
        return record.levelno >= ROOT_LEVEL and record.levelno < logging.ERROR


class StderrFilter(logging.Filter):
    def filter(self, record):
        return record.levelno >= logging.ERROR


formatter = CustomJsonFormatter("%(level)s %(name)s %(message)s")

stdoutHandler = logging.StreamHandler(stream=sys.stdout)
stdoutHandler.setFormatter(formatter)
stdoutHandler.addFilter(StdoutFilter())

stderrHandler = logging.StreamHandler(stream=sys.stderr)
stderrHandler.setFormatter(formatter)
stderrHandler.addFilter(StderrFilter())


root_logger = logging.getLogger()

root_logger.addHandler(stderrHandler)
root_logger.addHandler(stdoutHandler)

root_logger.setLevel(ROOT_LEVEL)

for i in range(0, 100):
    logging.debug('test')
    logging.info('test')
    logging.warning('test')
