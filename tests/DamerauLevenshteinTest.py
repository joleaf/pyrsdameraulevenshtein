import random
import time
import unittest

from pyxdameraulevenshtein import damerau_levenshtein_distance, normalized_damerau_levenshtein_distance
import pyrsdameraulevenshtein


class DamerauLevenshteinTest(unittest.TestCase):
    def test_against_pyxdameraulevenshtein_implementation(self):
        n = 1000
        x = 10
        a_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]
        b_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]

        for a, b in zip(a_lists, b_lists):
            result_pyx = damerau_levenshtein_distance(a, b)
            result_pyrs = pyrsdameraulevenshtein.int_distance(a, b)
            self.assertEqual(result_pyx, result_pyrs)
            result_pyx = normalized_damerau_levenshtein_distance(a, b)
            result_pyrs = pyrsdameraulevenshtein.int_normalized_distance(a, b)
            self.assertAlmostEqual(result_pyx, result_pyrs, places=4)

    def test_against_pyxdameraulevenshtein_implementation_performance(self):
        n = 100000
        x = 10
        a_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]
        b_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]

        tic = time.perf_counter()
        for a, b in zip(a_lists, b_lists):
            result = damerau_levenshtein_distance(a, b)
        toc = time.perf_counter()
        print(f"{toc - tic:0.4f} seconds, gold standard")

        tic = time.perf_counter()
        for a, b in zip(a_lists, b_lists):
            result = pyrsdameraulevenshtein.int_distance(a, b)
        toc = time.perf_counter()
        print(f"{toc - tic:0.4f} seconds, RUST implementation")

    def test_no_change(self):
        a_list = [1, 2, 3]
        b_list = [1, 2, 3]
        self.assertEqual(0, pyrsdameraulevenshtein.int_distance(a_list, b_list))
        self.assertAlmostEqual(0.0, pyrsdameraulevenshtein.int_normalized_distance(a_list, b_list), places=4)
        self.assertAlmostEqual(1.0, pyrsdameraulevenshtein.int_similarity(a_list, b_list), places=4)

    def test_small_change(self):
        a_list = [0, 1, 2, 3]
        b_list = [1, 2, 3, 4]
        self.assertEqual(2, pyrsdameraulevenshtein.int_distance(a_list, b_list))
        self.assertAlmostEqual(0.5, pyrsdameraulevenshtein.int_normalized_distance(a_list, b_list), places=4)
        self.assertAlmostEqual(0.5, pyrsdameraulevenshtein.int_similarity(a_list, b_list), places=4)

    def test_full_change(self):
        a_list = [0, 1, 2, 3]
        b_list = [5, 6, 7, 8]
        self.assertEqual(4, pyrsdameraulevenshtein.int_distance(a_list, b_list))
        self.assertAlmostEqual(1.0, pyrsdameraulevenshtein.int_normalized_distance(a_list, b_list), places=4)
        self.assertAlmostEqual(0.0, pyrsdameraulevenshtein.int_similarity(a_list, b_list), places=4)

    def test_one_empty_list(self):
        a_list = []
        b_list = [5, 6, 7, 8]
        self.assertEqual(4, pyrsdameraulevenshtein.int_distance(a_list, b_list))
        self.assertAlmostEqual(1.0, pyrsdameraulevenshtein.int_normalized_distance(a_list, b_list), places=4)
        self.assertAlmostEqual(0.0, pyrsdameraulevenshtein.int_similarity(a_list, b_list), places=4)

    def test_two_empty_lists(self):
        a_list = []
        b_list = []
        self.assertEqual(0, pyrsdameraulevenshtein.int_distance(a_list, b_list))
        self.assertAlmostEqual(0.0, pyrsdameraulevenshtein.int_normalized_distance(a_list, b_list), places=4)
        self.assertAlmostEqual(1.0, pyrsdameraulevenshtein.int_similarity(a_list, b_list), places=4)

    # def test_str_distance(self):
    #    a_str = "ABCE"
    #    b_str = "ABCD"
    #    self.assertEqual(1, pyrsdameraulevenshtein.str_distance(a_str, b_str))
    #    self.assertAlmostEqual(0.25, pyrsdameraulevenshtein.str_normalized_distance(a_str, b_str), places=4)
    #    self.assertAlmostEqual(0.75, pyrsdameraulevenshtein.str_similarity(a_str, b_str), places=4)


if __name__ == '__main__':
    unittest.main()