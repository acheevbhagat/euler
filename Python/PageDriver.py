from Page1 import Page1
import time
import Tools

startTime = time.clock()
pg = Page1()
lists = Tools.Tools()

print("Problem 1: " + str(pg.mult3_5()))  # problem 1 - 233168
print("Problem 2: " + str(pg.evenFib()))  # problem 2 - 4613732
print("Problem 3: " + str(pg.largestPrime()))  # problem 3 - 6857
print("Problem 4: " + str(pg.pal()))  # problem 4 - 906609
print("Problem 5: " + str(pg.smallestMult()))  # problem 55 - 232792560
print("Problem 6: " + str(pg.diffSquares()))  # problem 6 - 25164150
print("Problem 7: " + str(pg.numPrime()))  # probelm 7 - 104743
print("Problem 8: " + str(pg.productInSeries()))  # problem 8 - 23514624000
print("Problem 9: " + str(pg.pythTriplet()))  # problem 9 - 31875000
print("Problem 10: " + str(pg.sumPrime()))  # problem 10 - 142913828922
print("Problem 11: " + str(pg.gridProd()))  # problem 11 - 70600674
print("Problem 12: " + str(pg.triangularNumber()))  # problem 12 - 76576500
print("Problem 13: " + str(pg.largeSum()))  # problem 13 - 2493530798

print("Running time - %s seconds" % (time.clock() - startTime))
