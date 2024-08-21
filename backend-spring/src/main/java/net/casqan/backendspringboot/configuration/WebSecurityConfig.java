package net.casqan.backendspringboot.configuration;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpMethod;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configurers.AbstractHttpConfigurer;
import org.springframework.security.web.SecurityFilterChain;

@Configuration
@EnableWebSecurity
public class WebSecurityConfig {
    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http
                .authorizeHttpRequests(requests -> requests
                        .requestMatchers(HttpMethod.POST, "**").permitAll()
                        .requestMatchers(HttpMethod.PUT, "**").permitAll()
                        .requestMatchers(HttpMethod.DELETE, "**").permitAll()
                        .anyRequest().permitAll()
                )
                .csrf(AbstractHttpConfigurer::disable)
                .csrf(AbstractHttpConfigurer::disable);
        return http.build();
    }
}
