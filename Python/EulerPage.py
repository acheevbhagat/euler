from abc import ABC, abstractmethod
import Tools


class EulerPage(ABC):

    def __init__(self):
        self.tool = Tools.Tools()
