"""Tools for math problems."""

import copy
import time
from abc import ABC, abstractmethod


"""Ensure that only one copy of Tools is created"""


class Singleton(object):

    def __new__(cls):
        if not hasattr(cls, 'instance'):
            cls.instance = super(Singleton, cls).__new__(cls)
        return cls.instance


"""Math Tools"""


class Tools(Singleton):

    # determine if a number is prime
    def prime(self, n):
        if n <= 1:
            return False
        elif n <= 3:
            return True
        elif n % 2 == 0:  # if num divided by 2 has a remainder of 0, not prime
            return False
        root = int(self.sqrt(n)) + 1
        for i in range(3, root, 2):
            if n % i == 0:
                return False
        return True

    # determine if a number is a palindrome
    def palindrome(self, value):
        res = True
        valList = [int(x) for x in str(value)]
        for i in range(0, len(valList) - 1):
            if (valList[i] != valList[len(valList) - 1 - i]):
                res = False
        return res

    # find nth root of a number
    def root(self, value, power):
        return value ** (1.0 / power)

    # use Newton's method to find square root of a non perfect number
    def sqrt(self, val, approx=1):
        betterapprox = 0.5 * (approx + val/approx)
        if (betterapprox == approx):
            return approx
        else:
            return self.sqrt(val, betterapprox)

    # find factorial of a number
    def factorial(self, start):
        if (start > 1):
            return start * self.factorial(start - 1)
        else:
            return 1

    # find prime factors of a number
    def primeFactors(self, value):
        primfac = []
        d = 2
        while d ** 2 <= value:
            while (value % d) == 0:
                primfac.append(d)
                value //= d
            d += 1
        if value > 1:
            primfac.append(value)
        return primfac

    # sort a list using quicksort
    def sort(self, alist):
        self.__quicksort(alist, 0, len(alist)-1)

    def __quicksort(self, alist, first, last):
        if first < last:
            splitpoint = self.__partition(alist, first, last)
            self.__quicksort(alist, first, splitpoint - 1)
            self.__quicksort(alist, splitpoint + 1, last)

    def __partition(self, alist, first, last):
        pivotvalue = alist[first]
        leftmark = first + 1
        rightmark = last
        done = False
        while not done:
            while leftmark <= rightmark and alist[leftmark] <= pivotvalue:
                leftmark = leftmark + 1
            while alist[rightmark] >= pivotvalue and rightmark >= leftmark:
                rightmark = rightmark - 1
            if rightmark < leftmark:
                done = True
            else:
                temp = alist[leftmark]
                alist[leftmark] = alist[rightmark]
                alist[rightmark] = temp
        temp = alist[first]
        alist[first] = alist[rightmark]
        alist[rightmark] = temp
        return rightmark

    # find absolute value of a number
    def abs(self, val):
        if (val < 0):
            return val * -1
        else:
            return val

    # determine if three numbers satisfy pythagorean theorem
    def pyth(self, a, b, c):
        if a ** 2 + b ** 2 == c ** 2:
            return True
        else:
            return False

    # sieve of eratosthenes
    def sieve(self, limit):
        arr = [True] * limit
        res = []
        res.append(2)
        primeLimit = self.sqrt(limit)
        for i in range(3, limit + 1, 2):
            if arr[i]:
                res.append(i)
                if i < primeLimit:
                    j = i ** 2
                    while j < limit:
                        arr[j] = False
                        j += i
        return res

    # find all factors of a positive number
    def factors(self, num):
        res = []
        lim = self.root(num, 2)
        lim = int(lim) + 1
        for i in range(1, lim):
            if num % i == 0:
                res.append(i)
                res.append(num / i)
        self.sort(res)
        return res


class Timer(object):  # create timer object to time program

    def __init__(self):
        self.time = None

    def start(self):
        self.time = time.clock()

    def stop(self):
        tempTime = time.clock() - self.time
        self.time = tempTime

    def getTimer(self):
        return self.time

    def reset(self):
        self.time = None


"""Data Structures"""


class Node(object):  # general purpose node class for use by data structures

    def __init__(self, *args):
        self.data = []  # data[0] will always contain the data of the node
        if len(args) == 1 & type(args[0]) is Node:
            self.data = copy.deepcopy(args[0].getAll())
        else:
            for i in range(0, len(args)):  # subsequent values will contain
                self.data.append(args[i])  # directional info

    def setData(self, *args):
        if len(args) > 1:
            for i in range(0, len(args)):
                self.data[i] = args[i]
        else:
            self.data[0] = args[0]

    def getData(self, loc=0):
        return self.data[loc]

    def getAll(self):
        return self.data

    def __str__(self):
        return str(self.data[0])


class LinkedList(object):  # an unordered linked list

    def __init__(self, other=None):
        # default unordered
        # dummy head node implementation
        # no tail reference
        if other is None:
            self.head = Node(None, None)
        else:
            otherLength = other.length()
            for i in range(1, otherLength + 1):
                self.add(other.get(i))

    def add(self, val, pos=0):
        if pos == 0:
            temp = self.head
            new = Node(val, None)
            while temp.getData(1) is not None:
                temp = temp.getData(1)
            temp.setData(temp.getData(), new)
            return True
        elif (pos > 0) & (pos < self.length()):
            counter = 1
            prev = self.head
            temp = self.head.getData(1)
            while counter < pos:
                prev = temp
                temp = temp.getData(1)
                counter += 1
            new = Node(val, temp)
            prev.setData(prev.getData(), new)
            return True
        else:
            return False

    def delete(self, loc=0):
        if loc > 0:  # delete a value at a specific location
            prev = self.head
            temp = self.head.getData(1)
            counter = 1
            while counter != loc:
                counter += 1
                prev = temp
                temp = temp.getData(1)
            prev.setData(prev.getData(), temp.getData(1))
            return True
        elif loc == 0:  # reset the entire list
            self.head = Node(None, None)

    def get(self, loc=0):
        if loc > 0 & loc < self.length():
            counter = 0
            temp = self.head
            while counter != loc:
                counter += 1
                temp = temp.getData(1)
            return temp.getData()

    def length(self):
        counter = 0
        temp = self.head
        while temp.getData(1) is not None:
            counter += 1
            temp = temp.getData(1)
        return counter

    def __str__(self):
        res = ""
        if self.length() >= 1:
            temp = self.head.getData(1)
            res = res + str(temp.getData())
            while temp.getData(1) is not None:
                temp = temp.getData(1)
                res += " -> " + str(temp.getData())
        return res

    def __eq__(self, other):
        if str(self) == str(other):
            return True
        else:
            return False


class OrderedList(LinkedList):

    def __init__(self, other=None):
        super().__init__(other)

    def add(self, val):
        temp = self.head
        if temp.getData(1) is not None:
            cont = True
        else:
            cont = False
            temp.setData(None, Node(val, None))
            return True
        while cont:
            temp = temp.getData(1)
            if temp.getData(1) is not None:
                thisVal = temp.getData()
                nextVal = temp.getData(1).getData()
                if temp.getData() > val:
                    temp.setData(val, Node(temp.getData(), temp.getData(1)))
                    cont = False
                    return True
                elif (thisVal < val) & (nextVal > val):
                    temp.setData(temp.getData(), Node(val, temp.getData(1)))
                    cont = False
                    return True
                elif (thisVal == val):
                    temp.setData(temp.getData(), Node(val, temp.getData(1)))
                    cont = False
                    return True
            else:
                temp.setData(temp.getData(), Node(val, None))
                cont = False
                return True

class Queue(LinkedList):
	def __init__(self, other=None):
		super().__init__(other)

	def enqueue(self, val):
		self.add(val)

	def dequeue(self):
		if not self.isEmpty():
			res = self.get(1)
			self.delete(1)
			return res
		else:
			return None

	def peek(self):
		return get(1)

	def isEmpty(self):
		if self.length() == 0:
			return True
		return False

class Stack(LinkedList):
	def __init__(self, other=None):
		super().__init__(other)

	def push(self, val):
		self.add(val, 1)

	def pop(self):
		if not self.isEmpty():
			res = self.get(1)
			self.delete(1)
			return res
		else:
			return None

	def peek(self):
		return self.get(1)

	def isEmpty(self):
		if self.length() == 0:
			return True
		return False

class Tree(ABC):

    @abstractmethod
    def __init__(self, val):
        pass

    @abstractmethod
    def add(self, val):
        pass

    @abstractmethod
    def delete(self, val):
        pass


class BinarySearchTree(Tree):

    def __init__(self, val):
        # second parameter to Node object will be left subtree
        # third parameter to Node object will be right subtree
        self.root = Node(val, None, None)

    def add(self, val):
        if val is not None:
            return self.__addHelper(val, self.root)
        return False

    def __addHelper(self, val, node):
        if val > node.getData():
            if node.getData(2) is not None:
                self.__addHelper(val, node.getData(2))
            else:
                temp = node.getData()
                node.setData(temp, node.getData(1), Node(val, None, None))
                return True
        elif val < node.getData():
            if node.getData(1) is not None:
                self.__addHelper(val, node.getData(1))
            else:
                temp = node.getData()
                node.setData(temp, Node(val, None, None), node.getData(2))
                return True
        else:
            return False

    def __str__(self):  # unfinished
        temp = self.root
        pass
