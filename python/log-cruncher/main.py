from typing import Literal
from pydantic import BaseModel, Field, ValidationError
from datetime import datetime


class Log(BaseModel):
    timestamp: datetime
    level: Literal["INFO", "WARN", "ERROR", "DEBUG"]
    ip: str  # validate  x.x.x.x
    path: str  # validate starts with /
    method: Literal["GET", "POST", "PUT", "DELETE", "PATCH"]
    status: int = Field(gt=100, lt=500)
    duration: str  # Validate ms and parse


def main():
    """logcruncher pulls in a log files and crunches it!"""
    with open("./sample.log") as logFile:
        for line in logFile:
            vals = line.split(" ")
            try:
                log = Log.model_validate(
                    {
                        "timestamp": vals[0],
                        "level": vals[1],
                        "ip": vals[2],
                        "path": vals[3],
                        "method": vals[4],
                        "status": vals[5],
                        "duration": vals[6],
                    }
                )
                print(log)
            except ValidationError:
                pass


if __name__ == "__main__":
    main()
