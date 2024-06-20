import struct
from abc import ABCMeta, abstractmethod
import numpy as np

class Sensor(metaclass=ABCMeta):
    @property
    def ID(self)->int:
        raise NotImplementedError
    @abstractmethod
    def parse(self,array:list[int])->int:
        raise NotImplementedError
    @abstractmethod
    def database(self)->dict[str,list]:
        raise NotImplementedError

class ServoController(Sensor):
    def __init__(self,_id:int=0x10) -> None:
        super().__init__()
        self.id=_id
        self.raw_data={
            f"timestamp_{self.id:02x}" :[],
            f"rudder_{self.id:02x}"    :[],
            f"elevator_{self.id:02x}"  :[],
            f"voltage_{self.id:02x}"   :[],
            f"i_rudder_{self.id:02x}"  :[],
            f"i_elevator_{self.id:02x}":[],
            f"trim_{self.id:02x}"      :[],
            f"status_{self.id:02x}"    :[]
        }
    def parse(self, array: list[int])->int:
        _id,timestamp,rudder,elevator,voltage,i_rudder,i_elevator,trim,status=struct.unpack(">BxxxIffffffBxxx",bytes(array))
        self.raw_data[f"timestamp_{self.id:02x}" ].append(timestamp)
        self.raw_data[f"rudder_{self.id:02x}"    ].append(rudder)
        self.raw_data[f"elevator_{self.id:02x}"  ].append(elevator)
        self.raw_data[f"voltage_{self.id:02x}"   ].append(voltage)
        self.raw_data[f"i_rudder_{self.id:02x}"  ].append(i_rudder)
        self.raw_data[f"i_elevator_{self.id:02x}"].append(i_elevator)
        self.raw_data[f"trim_{self.id:02x}"      ].append(trim)
        self.raw_data[f"status_{self.id:02x}"    ].append(status)
        return 1
    @property
    def database(self)->dict[str,list]:
        return self.raw_data
    @property
    def ID(self)->int:
        return self.id

class Pitot(Sensor):
    def __init__(self,_id:int=0x21) -> None:
        super().__init__()
        self.id=_id
        self.raw_data={
            f"timestamp_{self.id:02x}":[],
            f"differential_pressure_{self.id:02x}":[],
            f"temperature_{self.id:02x}":[],
            f"velocity_{self.id:02x}":[],
        }
        self.id=_id
    def parse(self, array: list[int])->int:
        for n in range(len(array)//20):
            _id,timestamp,pressure,temperature,velocity=struct.unpack(">BxxxIfff",bytes(array[20*n:20*(n+1)]))
            self.raw_data[f"timestamp_{self.id:02x}"].append(timestamp)
            self.raw_data[f"differential_pressure_{self.id:02x}"].append(pressure)
            self.raw_data[f"temperature_{self.id:02x}"].append(temperature)
            self.raw_data[f"velocity_{self.id:02x}"].append(velocity)
        return len(array)//20
    @property
    def database(self)->dict[str,list]:
        return self.raw_data
    @property
    def ID(self)->int:
        return self.id

class Tachometer(Sensor):
    def __init__(self,_id:int=0x31) -> None:
        super().__init__()
        self.id=_id
        self.raw_data={
            f"timestamp_{self.id:02x}":[],
            f"cadence_{self.id:02x}":[],
        }
    def parse(self, array: list[int]):
        for n in range(len(array)//16):
            _id,timestamp,rpm,_,_=struct.unpack(">BxxxIhhf",bytes(array[16*n:16*(n+1)]))
            self.raw_data[f"timestamp_{self.id:02x}"].append(timestamp)
            self.raw_data[f"cadence_{self.id:02x}"].append(rpm/1.5)
        return len(array)//16
    @property
    def database(self)->dict[str,list]:
        return self.raw_data
    @property
    def ID(self)->int:
        return self.id

class IMU(Sensor):
    def __init__(self,_id:int=0x40) -> None:
        super().__init__()
        self.id=_id
        self.raw_data={
            f"timestamp_{self.id:02x}":[],
            f"w_{self.id:02x}":[],
            f"x_{self.id:02x}":[],
            f"y_{self.id:02x}":[],
            f"z_{self.id:02x}":[]
        }
    def parse(self, array: list[int]):
        for n in range(len(array)//20):
            _,timestamp,w,x,y,z=struct.unpack(">BxxxIhhhhxxxx",bytes(array[20*n:20*(n+1)]))
            w,x,y,z = w/16384.0,x/16384.0,y/16384.0,z/16384.0
            self.raw_data[f"timestamp_{self.id:02x}"].append(timestamp)
            self.raw_data[f"w_{self.id:02x}"].append(w)
            self.raw_data[f"x_{self.id:02x}"].append(x)
            self.raw_data[f"y_{self.id:02x}"].append(y)
            self.raw_data[f"z_{self.id:02x}"].append(z)
        return len(array)//20
    @property
    def database(self)->dict[str,list]:
        return self.raw_data
    @property
    def ID(self)->int:
        return self.id

class Altimeter(Sensor):
    def __init__(self,_id:int=0x52) -> None:
        super().__init__()
        self.id=_id
        self.raw_data={
            f"timestamp_{self.id:02x}":[],
            f"alt_{self.id:02x}":[],
        }
    def parse(self, array: list[int]):
        for n in range(len(array)//12):
            _id,timestamp,alt=struct.unpack(">BxxxIf",bytes(array[12*n:12*(n+1)]))
            self.raw_data[f"timestamp_{self.id:02x}"].append(timestamp)
            self.raw_data[f"alt_{self.id:02x}"].append(alt/100.0)
        return len(array)//12
    @property
    def database(self)->dict[str,list]:
        return self.raw_data
    @property
    def ID(self)->int:
        return self.id

class GPS(Sensor):
    def __init__(self,_id:int=0x60,zoom=14) -> None:
        super().__init__()
        self.id=_id
        self.raw_data={
            f"timestamp_{self.id:02x}":[],
            f"lat_{self.id:02x}":[],
            f"lon_{self.id:02x}":[],
            f"x_{self.id:02x}":[],
            f"y_{self.id:02x}":[]
        }
        self.zoom=zoom
    def parse(self, array: list[int]):
        for n in range(len(array)//24):
            _id,timestamp,lat,lon=struct.unpack(">BxxxIdd",bytes(array[24*n:24*(n+1)]))
            self.raw_data[f"timestamp_{self.id:02x}"].append(timestamp)
            self.raw_data[f"lat_{self.id:02x}"].append(lat)
            self.raw_data[f"lon_{self.id:02x}"].append(lon)
            self.raw_data[f"x_{self.id:02x}"].append(
                int((2.0**(self.zoom+7.0))*(lon/180.0+1))%256
            )
            self.raw_data[f"y_{self.id:02x}"].append(
                int((2.0**(self.zoom+7.0))/np.pi*(-np.arctanh(np.sin(np.radians(lat))) + np.arctanh(np.sin(np.radians(85.05112878)))))%256
            )
        return len(array)//24
    @property
    def database(self):
        return self.raw_data
    @property
    def ID(self)->int:
        return self.id

class Vane(Sensor):
    def __init__(self,_id:int=0x71) -> None:
        super().__init__()
        self.id=_id
        self.raw_data={
            f"timestamp_{self.id:02x}":[],
            f"angle_{self.id:02x}":[]
        }
    def parse(self, array: list[int]):
        for n in range(len(array)//12):
            _id,timestamp,angle=struct.unpack(">BxxxIf",bytes(array[12*n:12*(n+1)]))
            self.raw_data[f"timestamp_{self.id:02x}"].append(timestamp)
            self.raw_data[f"angle_{self.id:02x}"].append(angle)
        return len(array)//12
    @property
    def database(self):
        return self.raw_data
    @property
    def ID(self)->int:
        return self.id

class Barometer(Sensor):
    def __init__(self,_id:int=0x90) -> None:
        super().__init__()
        self.id=_id
        self.raw_data={
            f"timestamp_{self.id:02x}":[],
            f"pressure_{self.id:02x}":[],
            f"temperature_{self.id:02x}":[],
        }
    def parse(self, array: list[int]):
        for n in range(len(array)//16):
            _id,timestamp,pressure,temperature=struct.unpack(">BxxxIff",bytes(array[16*n:16*(n+1)]))
            self.raw_data[f"timestamp_{self.id:02x}"].append(timestamp)
            self.raw_data[f"pressure_{self.id:02x}"].append(pressure)
            self.raw_data[f"temperature_{self.id:02x}"].append(temperature)
        return len(array)//16
    @property
    def database(self):
        return self.raw_data
    @property
    def ID(self)->int:
        return self.id
