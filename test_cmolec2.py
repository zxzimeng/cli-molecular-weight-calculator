import subprocess
import unittest
from molmass import Formula

class TestCmolec2(unittest.TestCase):
    def run_cmolec2(self, formula):
        result = subprocess.run(['./target/release/cmolec2', formula], 
                              capture_output=True, text=True)
        return float(result.stdout.strip())

    def get_expected_mass(self, formula):
        # Convert underscore notation to parentheses for molmass
        formula = formula.replace('_', '(')
        if formula.count('(') > formula.count(')'):
            formula += ')'
        return float(Formula(formula).mass)

    def test_simple_molecules(self):
        test_cases = [
            'H2O',      # Water
            'CO2',      # Carbon dioxide
            'NH3',      # Ammonia
            'CH4',      # Methane
            'NaCl'      # Sodium chloride
        ]
        for formula in test_cases:
            with self.subTest(formula=formula):
                result = self.run_cmolec2(formula)
                expected = self.get_expected_mass(formula)
                self.assertAlmostEqual(result, expected, places=3)

    def test_complex_molecules(self):
        test_cases = [
            '_H2O_2',   # Water dimer
            '_OH_2Ca',  # Calcium hydroxide
            'H2SO4',    # Sulfuric acid
            '_CH3OH_3', # Methanol trimer
            '_NH3_4',   # Ammonia tetramer
            '_CO_2Fe'   # Iron carbonyl
        ]
        for formula in test_cases:
            with self.subTest(formula=formula):
                result = self.run_cmolec2(formula)
                expected = self.get_expected_mass(formula)
                self.assertAlmostEqual(result, expected, places=3)

    def test_invalid_input(self):
        invalid_formulas = ['Xx2', 'H2O_', '_H2O', '_H2O_', 'H2O__2']
        for formula in invalid_formulas:
            with self.subTest(formula=formula):
                result = subprocess.run(['./target/release/cmolec2', formula], 
                                     capture_output=True, text=True)
                self.assertNotEqual(result.returncode, 0)

if __name__ == '__main__':
    unittest.main()