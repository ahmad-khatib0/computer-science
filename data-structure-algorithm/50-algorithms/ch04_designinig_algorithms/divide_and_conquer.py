# 1. First, in order to use Apache Spark, we will create a runtime context of Apache Spark:
import findspark

findspark.init()

from pyspark.sql import SparkSession

spark = SparkSession.builder.master("local[*]").getOrCreate()
sc = spark.sparkContext

# 2. Now, let’s create a sample list containing some words. We will convert this list into Spark’s
#    native distributed data structure, called a Resilient Distributed Dataset (RDD):
wordsList = ["python", "java", "ottawa", "ottawa", "java", "news"]
wordsRDD = sc.parallelize(wordsList, 4)

print(wordsRDD.collect())
# ['python', 'java', 'ottawa', 'ottawa', 'java', 'news']

# 4. Now, let’s use a map function to convert the words into a key-value pair:
wordPairs = wordsRDD.map(lambda w: (w, 1))
print(wordPairs.collect())
# [('python', 1), ('java', 1), ('ottawa', 1), ('ottawa', 1), ('java', 1), ('news', 1)]

# 6: Let’s use the reduce function to aggregate and get the result:
wordCountsCollected = wordPairs.reduceByKey(lambda x, y: x + y)
print(wordCountsCollected.collect())
# [('python', 1), ('java', 2), ('ottawa', 2), ('news', 1)]


# This shows how we can use the divide-and-conquer strategy to count the number of words. Note
# that divide-and-conquer is useful when a problem can be divided into subproblems and each
# subproblem can at least be partially solved independently of other subproblems. It is not the best
# choice for algorithms that require intensive iterative processing, such as optimization algorithms.
# For such algorithms, dynamic programming is suitable,
