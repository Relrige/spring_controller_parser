@RestController
@RequestMapping("/api")
public class SampleController {
    @GetMapping("/hello")
    public String hello() { return "Hello"; }

    @PostMapping("/create")
    public void create(String body) { }
}
