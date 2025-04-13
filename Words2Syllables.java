import java.io.File;
import java.util.*;
import java.io.FileNotFoundException;
import java.util.stream.Collectors;


public class Words2Syllables {
  public static void main(String args[]) {
    HashMap<String, Integer> sylls = new HashMap<>();
    File wordFile = new File("./full.txt");
    try {
      Scanner wordScanner = new Scanner(wordFile);
      while (wordScanner.hasNextLine()) {
        String line = wordScanner.nextLine();
        for (int i = 0; i < line.length() - 1; i++) {
          String currentSyllable = line.substring(i, i + 2);
          addSyllable(sylls, currentSyllable);
        }
        for (int i = 0; i < line.length() - 2; i++) {
          String currentSyllable = line.substring(i, i + 3);
          addSyllable(sylls, currentSyllable);
        }
      }

      wordScanner.close();
      Map<String, Integer> sorted = sortByValue(sylls);

      for (Map.Entry<String, Integer> e : sorted.entrySet())
          System.out.println("Word: " + e.getKey() + "\tOcurrences: " + e.getValue());
    } catch (FileNotFoundException e) {
      System.out.println("an error occured");
      e.printStackTrace();
    }
  }

  static HashMap<String, Integer> sortByValue(HashMap<String, Integer> hm) {
      HashMap<String, Integer> temp = hm.entrySet().stream().sorted((i1, i2)-> 
          i1.getValue().compareTo(i2.getValue())).collect(
          Collectors.toMap(Map.Entry::getKey,Map.Entry::getValue,
          (e1, e2) -> e1, LinkedHashMap::new));

      return temp;
  }

  private static void addSyllable(HashMap<String, Integer> syllables, String input) {
    if (syllables.containsKey(input)) {
      syllables.put(input, syllables.get(input) + 1);
    } else if (!(input.contains("'") || input.contains("-"))) {
      syllables.put(input, 1);
    }
  }
}
