import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Function;
import java.util.stream.Collectors;

class ParallelLetterFrequency {
    private final String[] texts;

    ParallelLetterFrequency(String[] texts) {
        this.texts = texts;
    }

    Map<Character, Integer> countLetters() {
        Map<Character, AtomicInteger> frequency = new ConcurrentHashMap<>();
        ExecutorService executor = Executors.newFixedThreadPool(4);
        List<Callable<Void>> tasks = new ArrayList<>();
        for (String text : texts) {
            tasks.add(() -> {
                text.toLowerCase().chars()
                    .filter(Character::isLetter)
                    .forEach(c -> frequency.computeIfAbsent((char) c, k -> new AtomicInteger()).incrementAndGet());
                return null;
            });
        }
        try {
            executor.invokeAll(tasks);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        return frequency.entrySet().stream()
            .collect(Collectors.toMap(Map.Entry::getKey, e -> e.getValue().get()));
    }

}
