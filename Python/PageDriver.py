from Page1 import Page1
import time
import Tools

startTime = time.clock()
pg = Page1()
lists = Tools.Tools()

print("Problem 1: " + str(pg.mult3_5()))  # problem 1
print("Problem 2: " + str(pg.evenFib()))  # problem 2
print("Problem 3: " + str(pg.largestPrime()))  # problem 3
print("Problem 4: " + str(pg.pal()))  # problem 4
print("Problem 5: " + str(pg.smallestMult()))  # problem 5
print("Problem 6: " + str(pg.diffSquares()))  # problem 6
print("Problem 7: " + str(pg.numPrime()))  # probelm 7
print("Problem 8: " + str(pg.productInSeries()))  # problem 8
print("Problem 9: " + str(pg.pythTriplet()))  # problem 9
print("Problem 10: " + str(pg.sumPrime()))  # problem 10
print("Problem 11: " + str(pg.gridProd()))  # problem 11
print("Problem 12: " + str(pg.triangularNumber()))  # problem 12
print("Problem 13: " + str(pg.largeSum()))  # problem 13

print("Running time - %s seconds" % (time.clock() - startTime))
