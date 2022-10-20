import sys
import logging
from pythonjsonlogger import jsonlogger

class CustomJsonFormatter(jsonlogger.JsonFormatter):
    def add_fields(self, log_record, record, message_dict):
        super(CustomJsonFormatter, self).add_fields(log_record, record, message_dict)
        log_record["level"] = record.levelname
        log_record["name"] = record.name
        log_record["message"] = record.getMessage()
        if record.levelname == "ERROR" or record.levelname == "EXCEPTION":
            log_record["exception"] = record.exc_info
            log_record["stacktrace"] = record

formatter = CustomJsonFormatter("%(level)s %(name)s %(message)s")

stdoutHandler = logging.StreamHandler(stream=sys.stdout)
stdoutHandler.setLevel(logging.DEBUG)
stdoutHandler.setFormatter(formatter)

stderrHandler = logging.StreamHandler(stream=sys.stderr)
stderrHandler.setLevel(logging.ERROR)
stderrHandler.setFormatter(formatter)

root_logger = logging.getLogger()
root_logger.setLevel(logging.DEBUG)

root_logger.addHandler(stderrHandler)
root_logger.addHandler(stdoutHandler)
logging.debug('test')
logging.info('test')
logging.warning('test')
logging.error('test')
logging.critical('test')
