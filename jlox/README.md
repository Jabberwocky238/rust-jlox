# create
mvn archetype:generate -DgroupId=com.jlox -DartifactId=jlox -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false

# clean
mvn clean install

# run
cd jlox
java src\tool\GenerateAst.java src\main\java\com\jlox
mvn exec:java -Dexec.mainClass="com.jlox.Lox"


# examples
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/right/chapter8/scope1.lox"
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/right/chapter8/scope2.lox"
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/right/chapter8/scope_shadow.lox"
mvn exec:java -Dexec.mainClass="com.jlox.Lox" -Dexec.args="../scripts/right/chapter8/scope_nest.lox"