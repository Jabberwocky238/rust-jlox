mvn archetype:generate -DgroupId=com.jlox -DartifactId=jlox -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false

mvn compile

mvn exec:java -Dexec.mainClass="com.jlox.Lox"