import random
import time
import unittest

import jellyfish
import textdistance
#from fastDamerauLevenshtein import damerauLevenshtein
from pyxdameraulevenshtein import damerau_levenshtein_distance, normalized_damerau_levenshtein_distance
import pyrsdameraulevenshtein


class DamerauLevenshteinTest(unittest.TestCase):
    def test_against_pyxdameraulevenshtein_implementation_integers(self):
        n = 1000
        x = 10
        a_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]
        b_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]

        for a, b in zip(a_lists, b_lists):
            result_pyx = damerau_levenshtein_distance(a, b)
            result_pyrs = pyrsdameraulevenshtein.distance_int(a, b)
            self.assertEqual(result_pyx, result_pyrs)
            result_pyx = normalized_damerau_levenshtein_distance(a, b)
            result_pyrs = pyrsdameraulevenshtein.normalized_distance_int(a, b)
            self.assertAlmostEqual(result_pyx, result_pyrs, places=4)

    def test_against_pyxdameraulevenshtein_implementation_strings(self):
        n = 1000
        x = 10
        a_strings = [
            "".join(random.sample(list(chr(ord("A") + i) for i in range(x)), k=x, counts=[x for i in range(x)]))
            for y in range(n)]
        b_strings = [
            "".join(random.sample(list(chr(ord("A") + i) for i in range(x)), k=x, counts=[x for i in range(x)]))
            for y in range(n)]
        for a, b in zip(a_strings, b_strings):
            result_pyx = damerau_levenshtein_distance(a, b)
            result_pyrs = pyrsdameraulevenshtein.distance_unicode(a, b)
            self.assertEqual(result_pyx, result_pyrs)
            result_pyx = normalized_damerau_levenshtein_distance(a, b)
            result_pyrs = pyrsdameraulevenshtein.normalized_distance_unicode(a, b)
            self.assertAlmostEqual(result_pyx, result_pyrs, places=4)

    def test_list_performance_against_other_implementations(self):
        n = 100000
        x = 10

        print("Int lists:")
        a_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]
        b_lists = [random.sample(list(range(x)), k=x, counts=[x for i in range(x)]) for i in range(n)]
        tic = time.perf_counter()
        for a, b in zip(a_lists, b_lists):
            result = pyrsdameraulevenshtein.distance_int(a, b)
        toc = time.perf_counter()
        print(f"{toc - tic:0.4f} seconds, THIS implementation")
        tic = time.perf_counter()
        for a, b in zip(a_lists, b_lists):
            result = damerau_levenshtein_distance(a, b)
        toc = time.perf_counter()
        print(f"{toc - tic:0.4f} seconds, pyxdameraulevenshtein")
        #tic = time.perf_counter()
        #for a, b in zip(a_lists, b_lists):
        #    result = damerauLevenshtein(a, b, similarity=False)
        #toc = time.perf_counter()
        #print(f"{toc - tic:0.4f} seconds, fastDamerauLevenshtein")

    def test_string_performance_against_other_implementations(self):
        n = 100000
        x = 10

        print("Strings:")
        a_strings = [
            "".join(random.sample(list(chr(ord("A") + i) for i in range(x)), k=x, counts=[x for i in range(x)]))
            for y in range(n)]
        b_strings = [
            "".join(random.sample(list(chr(ord("A") + i) for i in range(x)), k=x, counts=[x for i in range(x)]))
            for y in range(n)]
        tic = time.perf_counter()
        for a, b in zip(a_strings, b_strings):
            result = pyrsdameraulevenshtein.distance_unicode(a, b)
        toc = time.perf_counter()
        print(f"{toc - tic:0.4f} seconds, THIS implementation")
        tic = time.perf_counter()
        for a, b in zip(a_strings, b_strings):
            result = damerau_levenshtein_distance(a, b)
        toc = time.perf_counter()
        print(f"{toc - tic:0.4f} seconds, pyxdameraulevenshtein")
        #tic = time.perf_counter()
        #for a, b in zip(a_strings, b_strings):
        #    result = damerauLevenshtein(a, b, similarity=False)
        #toc = time.perf_counter()
        #print(f"{toc - tic:0.4f} seconds, fastDamerauLevenshtein")
        tic = time.perf_counter()
        for a, b in zip(a_strings, b_strings):
            result = jellyfish.damerau_levenshtein_distance(a, b)
        toc = time.perf_counter()
        print(f"{toc - tic:0.4f} seconds, jellyfish.damerau_levenshtein_distance")
        tic = time.perf_counter()
        for a, b in zip(a_strings, b_strings):
            result = textdistance.DamerauLevenshtein(a, b)
        toc = time.perf_counter()
        print(f"{toc - tic:0.4f} seconds, textdistance.DamerauLevenshtein")

    def test_no_change(self):
        a_list = [1, 2, 3]
        b_list = [1, 2, 3]
        self.assertEqual(0, pyrsdameraulevenshtein.distance_int(a_list, b_list))
        self.assertAlmostEqual(0.0, pyrsdameraulevenshtein.normalized_distance_int(a_list, b_list), places=4)
        self.assertAlmostEqual(1.0, pyrsdameraulevenshtein.similarity_int(a_list, b_list), places=4)

    def test_small_change(self):
        a_list = [0, 1, 2, 3]
        b_list = [1, 2, 3, 4]
        self.assertEqual(2, pyrsdameraulevenshtein.distance_int(a_list, b_list))
        self.assertAlmostEqual(0.5, pyrsdameraulevenshtein.normalized_distance_int(a_list, b_list), places=4)
        self.assertAlmostEqual(0.5, pyrsdameraulevenshtein.similarity_int(a_list, b_list), places=4)

    def test_full_change(self):
        a_list = [0, 1, 2, 3]
        b_list = [5, 6, 7, 8]
        self.assertEqual(4, pyrsdameraulevenshtein.distance_int(a_list, b_list))
        self.assertAlmostEqual(1.0, pyrsdameraulevenshtein.normalized_distance_int(a_list, b_list), places=4)
        self.assertAlmostEqual(0.0, pyrsdameraulevenshtein.similarity_int(a_list, b_list), places=4)

    def test_one_empty_list(self):
        a_list = []
        b_list = [5, 6, 7, 8]
        self.assertEqual(4, pyrsdameraulevenshtein.distance_int(a_list, b_list))
        self.assertAlmostEqual(1.0, pyrsdameraulevenshtein.normalized_distance_int(a_list, b_list), places=4)
        self.assertAlmostEqual(0.0, pyrsdameraulevenshtein.similarity_int(a_list, b_list), places=4)

    def test_two_empty_lists(self):
        a_list = []
        b_list = []
        self.assertEqual(0, pyrsdameraulevenshtein.distance_int(a_list, b_list))
        self.assertAlmostEqual(0.0, pyrsdameraulevenshtein.normalized_distance_int(a_list, b_list), places=4)
        self.assertAlmostEqual(1.0, pyrsdameraulevenshtein.similarity_int(a_list, b_list), places=4)

    def test_str_distance(self):
        a_str = list("ABCE")
        b_str = list("ABCD")
        self.assertEqual(1, pyrsdameraulevenshtein.distance_str(a_str, b_str))
        self.assertAlmostEqual(0.25, pyrsdameraulevenshtein.normalized_distance_str(a_str, b_str), places=4)
        self.assertAlmostEqual(0.75, pyrsdameraulevenshtein.similarity_str(a_str, b_str), places=4)

    def test_unicode_distance(self):
        a_str = "ABCE"
        b_str = "ABCD"
        self.assertEqual(1, pyrsdameraulevenshtein.distance_unicode(a_str, b_str))
        self.assertAlmostEqual(0.25, pyrsdameraulevenshtein.normalized_distance_unicode(a_str, b_str), places=4)
        self.assertAlmostEqual(0.75, pyrsdameraulevenshtein.similarity_unicode(a_str, b_str), places=4)


if __name__ == '__main__':
    unittest.main()
